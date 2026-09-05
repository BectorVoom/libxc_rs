//! GGA_C_PBE fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_pbe.c`
//! by tools/translate_rayon/from_maple.py, then rewritten to
//! `wide::f64x8` by simd.py. Eight grid points per step; every lane runs maple2c's expression
//! sequence in its original order.
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]
use libxc_rkernel_math::constants::*;
use libxc_rkernel_math::simd;
use libxc_rkernel_math::wide::{f64x8, CmpEq, CmpGe, CmpGt, CmpLe, CmpLt, CmpNe};

const V_ZERO: f64x8 = f64x8::new([0.0; 8]);
const V_ONE: f64x8 = f64x8::new([1.0; 8]);

// Transcendentals in exact mode come from `libxc_rkernel_math::simd`,
// which is bit-identical / correctly-rounded per lane to the scalar calls
// the scalar kernel makes. In exact mode, the SIMD kernel produces output
// bit-identical to its scalar form.

/// Load 8 consecutive grid points.
///
/// The tail is padded by repeating the last element, not by zero-filling:
/// these formulas divide by rho, so a zero lane would raise inf/NaN in lanes
/// whose results are then discarded -- harmless to the answer, but it makes
/// any real NaN impossible to spot while debugging.
#[inline(always)]
fn load(s: &[f64], ip: usize, np: usize) -> f64x8 {
    if ip + 8 <= np {
        let mut b = [0.0f64; 8];
        b.copy_from_slice(&s[ip..ip + 8]);
        f64x8::new(b)
    } else {
        let mut b = [s[np - 1]; 8];
        b[..np - ip].copy_from_slice(&s[ip..np]);
        f64x8::new(b)
    }
}

/// Accumulate 8 consecutive grid points into an output array.
///
/// `+=`, not `=`. The scalar kernel writes `out[ip] += v`; a plain store is a
/// different operation in two ways. It keeps the sign of a negative zero where
/// `0.0 + -0.0` gives `+0.0` -- a bit difference the fingerprint gate reports
/// as a rejection even though no value changed (`gga_x_pbepow fxc` was
/// rejected on exactly this, 273 of 200,000 `v2sigma2` elements) -- and it
/// would discard whatever a caller had already put in the buffer.
#[inline(always)]
fn store_add(s: &mut [f64], ip: usize, m: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let mut b = [0.0f64; 8];
        b.copy_from_slice(&s[ip..ip + 8]);
        let r: [f64; 8] = (f64x8::new(b) + acc).into();
        s[ip..ip + 8].copy_from_slice(&r);
    } else {
        for k in 0..m {
            s[ip + k] += a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_pbe_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    param_gamma: f64,
    param_BB: f64,
    param_beta: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_gamma = f64x8::splat(param_gamma);
    let param_BB = f64x8::splat(param_BB);
    let param_beta = f64x8::splat(param_beta);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_vsigma = V_ZERO;
        let mut acc_v2rho2 = V_ZERO;
        let mut acc_v2rhosigma = V_ZERO;
        let mut acc_v2sigma2 = V_ZERO;
        {
            let t1 = f64x8::splat(M_CBRT3);
            let t2 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t3 = (simd::cbrt(t2));
            let t4 = t1 * t3;
            let t5 = f64x8::splat(M_CBRT4);
            let t6 = t5 * t5;
            let t7 = (simd::cbrt(v_rho));
            let t10 = t4 * t6 / t7;
            let t12 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t10;
            let t13 = ((t10).sqrt());
            let t16 = ((t10) * (t10).sqrt());
            let t18 = t1 * t1;
            let t19 = t3 * t3;
            let t20 = t18 * t19;
            let t21 = t7 * t7;
            let t24 = t20 * t5 / t21;
            let t26 = f64x8::splat(3.79785) * t13 + f64x8::splat(0.8969) * t10 + f64x8::splat(0.204775) * t16 + f64x8::splat(0.123235) * t24;
            let t29 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t26;
            let t30 = (simd::ln(t29));
            let t32 = f64x8::splat(0.0621814) * t12 * t30;
            let t33 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t34 = (simd::cbrt(zeta_threshold));
            let t36 = ((t33).select(t34 * zeta_threshold, f64x8::splat(1.0)));
            let t39 = f64x8::splat(M_CBRT2);
            let t43 = (f64x8::splat(2.0) * t36 - f64x8::splat(2.0)) / (f64x8::splat(2.0) * t39 - f64x8::splat(2.0));
            let t45 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t10;
            let t50 = f64x8::splat(5.1785) * t13 + f64x8::splat(0.905775) * t10 + f64x8::splat(0.1100325) * t16 + f64x8::splat(0.1241775) * t24;
            let t53 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t50;
            let t54 = (simd::ln(t53));
            let t57 = f64x8::splat(0.0197516734986138) * t43 * t45 * t54;
            let t58 = t34 * t34;
            let t59 = ((t33).select(t58, f64x8::splat(1.0)));
            let t60 = t59 * t59;
            let t61 = t60 * t59;
            let t62 = param_gamma * t61;
            let t63 = v_rho * v_rho;
            let t65 = f64x8::splat(1.0) / t7 / t63;
            let t68 = f64x8::splat(1.0) / t60;
            let t70 = f64x8::splat(1.0) / t3;
            let t72 = t68 * t18 * t70 * t5;
            let t75 = param_BB * param_beta;
            let t76 = f64x8::splat(1.0) / param_gamma;
            let t79 = f64x8::splat(1.0) / t61;
            let t81 = (simd::exp(-(-t32 + t57) * t76 * t79));
            let t82 = t81 - f64x8::splat(1.0);
            let t83 = f64x8::splat(1.0) / t82;
            let t84 = t76 * t83;
            let t85 = v_sigma * v_sigma;
            let t87 = t75 * t84 * t85;
            let t88 = t63 * t63;
            let t90 = f64x8::splat(1.0) / t21 / t88;
            let t91 = t39 * t39;
            let t92 = t90 * t91;
            let t93 = t60 * t60;
            let t94 = f64x8::splat(1.0) / t93;
            let t95 = t92 * t94;
            let t96 = f64x8::splat(1.0) / t19;
            let t97 = t1 * t96;
            let t98 = t97 * t6;
            let t99 = t95 * t98;
            let t102 = v_sigma * t65 * t39 * t72 / f64x8::splat(96.0) + t87 * t99 / f64x8::splat(3072.0);
            let t103 = param_beta * t102;
            let t104 = param_beta * t76;
            let t107 = t104 * t83 * t102 + f64x8::splat(1.0);
            let t108 = f64x8::splat(1.0) / t107;
            let t109 = t76 * t108;
            let t111 = t103 * t109 + f64x8::splat(1.0);
            let t112 = (simd::ln(t111));
            let t113 = t62 * t112;
            let tzk0 = -t32 + t57 + t113;
            acc_zk = tzk0;
            let t115 = f64x8::splat(1.0) / t7 / v_rho;
            let t116 = t6 * t115;
            let t118 = t4 * t116 * t30;
            let t119 = f64x8::splat(0.0011073470983333333) * t118;
            let t120 = t26 * t26;
            let t121 = f64x8::splat(1.0) / t120;
            let t122 = t12 * t121;
            let t124 = f64x8::splat(1.0) / t13 * t1;
            let t125 = t3 * t6;
            let t126 = t125 * t115;
            let t127 = t124 * t126;
            let t129 = t4 * t116;
            let t131 = ((t10).sqrt());
            let t132 = t131 * t1;
            let t133 = t132 * t126;
            let t138 = t20 * t5 / t21 / v_rho;
            let t140 = -f64x8::splat(0.632975) * t127 - f64x8::splat(0.29896666666666666) * t129 - f64x8::splat(0.1023875) * t133 - f64x8::splat(0.08215666666666667) * t138;
            let t141 = f64x8::splat(1.0) / t29;
            let t142 = t140 * t141;
            let t143 = t122 * t142;
            let t144 = f64x8::splat(1.0) * t143;
            let t145 = t43 * t1;
            let t148 = t145 * t125 * t115 * t54;
            let t149 = f64x8::splat(0.00018311447306006544) * t148;
            let t150 = t43 * t45;
            let t151 = t50 * t50;
            let t152 = f64x8::splat(1.0) / t151;
            let t157 = -f64x8::splat(0.8630833333333333) * t127 - f64x8::splat(0.301925) * t129 - f64x8::splat(0.05501625) * t133 - f64x8::splat(0.082785) * t138;
            let t159 = f64x8::splat(1.0) / t53;
            let t160 = t152 * t157 * t159;
            let t161 = t150 * t160;
            let t162 = f64x8::splat(0.5848223622634646) * t161;
            let t163 = t63 * v_rho;
            let t165 = f64x8::splat(1.0) / t7 / t163;
            let t170 = param_gamma * param_gamma;
            let t171 = f64x8::splat(1.0) / t170;
            let t172 = t75 * t171;
            let t173 = t82 * t82;
            let t174 = f64x8::splat(1.0) / t173;
            let t175 = t174 * t85;
            let t176 = t175 * t90;
            let t177 = t172 * t176;
            let t179 = f64x8::splat(1.0) / t93 / t61;
            let t180 = t91 * t179;
            let t181 = t180 * t1;
            let t182 = t96 * t6;
            let t183 = t119 + t144 - t149 - t162;
            let t184 = t183 * t81;
            let t185 = t182 * t184;
            let t186 = t181 * t185;
            let t189 = t88 * v_rho;
            let t191 = f64x8::splat(1.0) / t21 / t189;
            let t192 = t191 * t91;
            let t193 = t192 * t94;
            let t194 = t193 * t98;
            let t197 = -f64x8::splat(7.0) / f64x8::splat(288.0) * v_sigma * t165 * t39 * t72 + t177 * t186 / f64x8::splat(3072.0) - f64x8::splat(7.0) / f64x8::splat(4608.0) * t87 * t194;
            let t198 = param_beta * t197;
            let t200 = t107 * t107;
            let t201 = f64x8::splat(1.0) / t200;
            let t202 = t76 * t201;
            let t204 = param_beta * t171 * t174;
            let t206 = t79 * t81;
            let t211 = t204 * t102 * t183 * t206 + t104 * t83 * t197;
            let t212 = t202 * t211;
            let t214 = -t103 * t212 + t198 * t109;
            let t215 = f64x8::splat(1.0) / t111;
            let t217 = t62 * t214 * t215;
            let tvrho0 = -t32 + t57 + t113 + v_rho * (t119 + t144 - t149 - t162 + t217);
            acc_vrho = tvrho0;
            let t220 = v_rho * param_gamma;
            let t224 = t18 * t70 * t5;
            let t228 = t75 * t84 * v_sigma;
            let t231 = t65 * t39 * t68 * t224 / f64x8::splat(96.0) + t228 * t99 / f64x8::splat(1536.0);
            let t232 = param_beta * t231;
            let t234 = param_beta * param_beta;
            let t235 = t234 * t102;
            let t236 = t235 * t171;
            let t237 = t201 * t83;
            let t238 = t237 * t231;
            let t240 = t232 * t109 - t236 * t238;
            let tvsigma0 = t220 * t61 * t240 * t215;
            acc_vsigma = tvsigma0;
            let t248 = t6 * t65;
            let t250 = t4 * t248 * t30;
            let t251 = f64x8::splat(0.0014764627977777779) * t250;
            let t252 = t4 * t6;
            let t253 = t115 * t121;
            let t255 = t252 * t253 * t142;
            let t256 = f64x8::splat(0.035616666666666665) * t255;
            let t257 = t120 * t26;
            let t258 = f64x8::splat(1.0) / t257;
            let t259 = t12 * t258;
            let t260 = t140 * t140;
            let t261 = t260 * t141;
            let t262 = t259 * t261;
            let t263 = f64x8::splat(2.0) * t262;
            let t266 = f64x8::splat(1.0) / t13 / t10 * t18;
            let t267 = t19 * t5;
            let t269 = f64x8::splat(1.0) / t21 / t63;
            let t270 = t267 * t269;
            let t271 = t266 * t270;
            let t273 = t125 * t65;
            let t274 = t124 * t273;
            let t276 = t4 * t248;
            let t278 = f64x8::splat(1.0)/((t10).sqrt());
            let t279 = t278 * t18;
            let t280 = t279 * t270;
            let t282 = t132 * t273;
            let t285 = t20 * t5 * t269;
            let t287 = -f64x8::splat(0.4219833333333333) * t271 + f64x8::splat(0.8439666666666666) * t274 + f64x8::splat(0.3986222222222222) * t276 + f64x8::splat(0.06825833333333334) * t280 + f64x8::splat(0.13651666666666668) * t282 + f64x8::splat(0.1369277777777778) * t285;
            let t288 = t287 * t141;
            let t289 = t122 * t288;
            let t290 = f64x8::splat(1.0) * t289;
            let t291 = t120 * t120;
            let t292 = f64x8::splat(1.0) / t291;
            let t293 = t12 * t292;
            let t294 = t29 * t29;
            let t295 = f64x8::splat(1.0) / t294;
            let t296 = t260 * t295;
            let t297 = t293 * t296;
            let t298 = f64x8::splat(16.081979498692537) * t297;
            let t301 = t145 * t125 * t65 * t54;
            let t302 = f64x8::splat(0.00024415263074675396) * t301;
            let t303 = t43 * t4;
            let t305 = t303 * t116 * t160;
            let t306 = f64x8::splat(0.01084358130030174) * t305;
            let t307 = t151 * t50;
            let t308 = f64x8::splat(1.0) / t307;
            let t309 = t157 * t157;
            let t311 = t308 * t309 * t159;
            let t312 = t150 * t311;
            let t313 = f64x8::splat(1.1696447245269292) * t312;
            let t320 = -f64x8::splat(0.5753888888888888) * t271 + f64x8::splat(1.1507777777777777) * t274 + f64x8::splat(0.4025666666666667) * t276 + f64x8::splat(0.0366775) * t280 + f64x8::splat(0.073355) * t282 + f64x8::splat(0.137975) * t285;
            let t322 = t152 * t320 * t159;
            let t323 = t150 * t322;
            let t324 = f64x8::splat(0.5848223622634646) * t323;
            let t325 = t151 * t151;
            let t326 = f64x8::splat(1.0) / t325;
            let t327 = t326 * t309;
            let t328 = t53 * t53;
            let t329 = f64x8::splat(1.0) / t328;
            let t330 = t327 * t329;
            let t331 = t150 * t330;
            let t332 = f64x8::splat(17.315859105681465) * t331;
            let t334 = f64x8::splat(1.0) / t7 / t88;
            let t340 = f64x8::splat(1.0) / t170 / param_gamma;
            let t341 = t75 * t340;
            let t343 = f64x8::splat(1.0) / t173 / t82;
            let t344 = t343 * t85;
            let t345 = t344 * t90;
            let t346 = t341 * t345;
            let t347 = t93 * t93;
            let t349 = f64x8::splat(1.0) / t347 / t60;
            let t351 = t91 * t349 * t1;
            let t352 = t183 * t183;
            let t353 = t81 * t81;
            let t354 = t352 * t353;
            let t356 = t351 * t182 * t354;
            let t359 = t175 * t191;
            let t360 = t172 * t359;
            let t363 = -t251 - t256 - t263 + t290 + t298 + t302 + t306 + t313 - t324 - t332;
            let t364 = t363 * t81;
            let t366 = t181 * t182 * t364;
            let t369 = t341 * t176;
            let t370 = t352 * t81;
            let t372 = t351 * t182 * t370;
            let t375 = t88 * t63;
            let t377 = f64x8::splat(1.0) / t21 / t375;
            let t380 = t377 * t91 * t94 * t98;
            let t383 = f64x8::splat(35.0) / f64x8::splat(432.0) * v_sigma * t334 * t39 * t72 + t346 * t356 / f64x8::splat(1536.0) - f64x8::splat(7.0) / f64x8::splat(2304.0) * t360 * t186 + t177 * t366 / f64x8::splat(3072.0) - t369 * t372 / f64x8::splat(3072.0) + f64x8::splat(119.0) / f64x8::splat(13824.0) * t87 * t380;
            let t384 = param_beta * t383;
            let t389 = f64x8::splat(1.0) / t200 / t107;
            let t390 = t76 * t389;
            let t391 = t211 * t211;
            let t392 = t390 * t391;
            let t395 = param_beta * t340;
            let t396 = t395 * t343;
            let t397 = t102 * t352;
            let t399 = f64x8::splat(1.0) / t93 / t60;
            let t400 = t399 * t353;
            let t411 = t395 * t174;
            let t412 = t399 * t81;
            let t417 = t204 * t102 * t363 * t206 + f64x8::splat(2.0) * t204 * t197 * t183 * t206 + t104 * t83 * t383 + f64x8::splat(2.0) * t396 * t397 * t400 - t411 * t397 * t412;
            let t418 = t202 * t417;
            let t420 = f64x8::splat(2.0) * t103 * t392 - t103 * t418 + t384 * t109 - f64x8::splat(2.0) * t198 * t212;
            let t422 = t62 * t420 * t215;
            let t423 = t214 * t214;
            let t424 = t111 * t111;
            let t425 = f64x8::splat(1.0) / t424;
            let t427 = t62 * t423 * t425;
            let t428 = -t251 - t256 - t263 + t290 + t298 + t302 + t306 + t313 - t324 - t332 + t422 - t427;
            let tv2rho20 = f64x8::splat(0.0022146941966666666) * t118 + f64x8::splat(2.0) * t143 - f64x8::splat(0.0003662289461201309) * t148 - f64x8::splat(1.1696447245269292) * t161 + f64x8::splat(2.0) * t217 + v_rho * t428;
            acc_v2rho2 = tv2rho20;
            let t436 = t174 * v_sigma;
            let t437 = t436 * t90;
            let t438 = t172 * t437;
            let t443 = -f64x8::splat(7.0) / f64x8::splat(288.0) * t165 * t39 * t68 * t224 + t438 * t186 / f64x8::splat(1536.0) - f64x8::splat(7.0) / f64x8::splat(2304.0) * t228 * t194;
            let t444 = param_beta * t443;
            let t447 = t234 * t197;
            let t448 = t447 * t171;
            let t450 = t389 * t83;
            let t451 = t231 * t211;
            let t452 = t450 * t451;
            let t455 = t340 * t201;
            let t456 = t235 * t455;
            let t457 = t174 * t231;
            let t459 = t183 * t79 * t81;
            let t460 = t457 * t459;
            let t462 = t237 * t443;
            let t464 = t444 * t109 - t232 * t212 + f64x8::splat(2.0) * t236 * t452 - t236 * t462 - t448 * t238 - t456 * t460;
            let t468 = t220 * t61;
            let t469 = t240 * t425;
            let t470 = t469 * t214;
            let tv2rhosigma0 = t220 * t61 * t464 * t215 + t62 * t240 * t215 - t468 * t470;
            acc_v2rhosigma = tv2rhosigma0;
            let t472 = t234 * param_BB;
            let t473 = t171 * t83;
            let t479 = t91 * t94 * t1 * t182 * t108;
            let t482 = t231 * t231;
            let t483 = t234 * t482;
            let t485 = t171 * t201 * t83;
            let t488 = t234 * param_beta;
            let t489 = t488 * t102;
            let t490 = t489 * t340;
            let t491 = t389 * t174;
            let t492 = t491 * t482;
            let t496 = t201 * t174 * param_BB;
            let t497 = t490 * t496;
            let t500 = t472 * t473 * t90 * t479 / f64x8::splat(1536.0) - f64x8::splat(2.0) * t483 * t485 + f64x8::splat(2.0) * t490 * t492 - t497 * t99 / f64x8::splat(1536.0);
            let t504 = t240 * t240;
            let tv2sigma20 = t220 * t61 * t500 * t215 - t220 * t61 * t504 * t425;
            acc_v2sigma2 = tv2sigma20;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        store_add(v2rho2, ip, m, acc_v2rho2);
        store_add(v2rhosigma, ip, m, acc_v2rhosigma);
        store_add(v2sigma2, ip, m, acc_v2sigma2);
        ip += 8;
    }
}
