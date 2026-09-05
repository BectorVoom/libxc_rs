//! GGA_C_ZPBEINT fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_zpbeint.c`
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
pub fn gga_c_zpbeint_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    param_alpha: f64,
    param_beta: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_alpha = f64x8::splat(param_alpha);
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
            let t60 = ((v_sigma).sqrt());
            let t61 = t60 * v_sigma;
            let t62 = param_alpha * t61;
            let t63 = v_rho * v_rho;
            let t64 = t63 * t63;
            let t65 = f64x8::splat(1.0) / t64;
            let t66 = t59 * t59;
            let t67 = t66 * t59;
            let t68 = f64x8::splat(1.0) / t67;
            let t71 = f64x8::splat(1.0) / t13 / t10;
            let t75 = (simd::pow(t59, t62 * t65 * t68 * t71 / f64x8::splat(16.0)));
            let t76 = (simd::ln(f64x8::splat(2.0)));
            let t77 = f64x8::splat(1.0) - t76;
            let t78 = t75 * t77;
            let t79 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t80 = f64x8::splat(1.0) / t79;
            let t81 = t80 * t67;
            let t83 = f64x8::splat(1.0) / t7 / t63;
            let t86 = f64x8::splat(1.0) / t66;
            let t88 = f64x8::splat(1.0) / t3;
            let t90 = t86 * t18 * t88 * t5;
            let t93 = f64x8::splat(1.0) / t77;
            let t94 = param_beta * t93;
            let t99 = (simd::exp(-(-t32 + t57) * t93 * t79 * t68));
            let t100 = t99 - f64x8::splat(1.0);
            let t101 = f64x8::splat(1.0) / t100;
            let t102 = t79 * t101;
            let t103 = v_sigma * v_sigma;
            let t105 = t94 * t102 * t103;
            let t107 = f64x8::splat(1.0) / t21 / t64;
            let t108 = t39 * t39;
            let t109 = t107 * t108;
            let t110 = t66 * t66;
            let t111 = f64x8::splat(1.0) / t110;
            let t112 = t109 * t111;
            let t113 = f64x8::splat(1.0) / t19;
            let t114 = t1 * t113;
            let t115 = t114 * t6;
            let t116 = t112 * t115;
            let t119 = v_sigma * t83 * t39 * t90 / f64x8::splat(96.0) + t105 * t116 / f64x8::splat(3072.0);
            let t120 = param_beta * t119;
            let t124 = t94 * t102 * t119 + f64x8::splat(1.0);
            let t125 = f64x8::splat(1.0) / t124;
            let t126 = t93 * t79 * t125;
            let t128 = t120 * t126 + f64x8::splat(1.0);
            let t129 = (simd::ln(t128));
            let t131 = t78 * t81 * t129;
            let tzk0 = -t32 + t57 + t131;
            acc_zk = tzk0;
            let t133 = f64x8::splat(1.0) / t7 / v_rho;
            let t134 = t6 * t133;
            let t136 = t4 * t134 * t30;
            let t137 = f64x8::splat(0.0011073470983333333) * t136;
            let t138 = t26 * t26;
            let t139 = f64x8::splat(1.0) / t138;
            let t140 = t12 * t139;
            let t142 = f64x8::splat(1.0) / t13 * t1;
            let t143 = t3 * t6;
            let t144 = t143 * t133;
            let t145 = t142 * t144;
            let t147 = t4 * t134;
            let t149 = ((t10).sqrt());
            let t150 = t149 * t1;
            let t151 = t150 * t144;
            let t155 = t5 / t21 / v_rho;
            let t156 = t20 * t155;
            let t158 = -f64x8::splat(0.632975) * t145 - f64x8::splat(0.29896666666666666) * t147 - f64x8::splat(0.1023875) * t151 - f64x8::splat(0.08215666666666667) * t156;
            let t159 = f64x8::splat(1.0) / t29;
            let t160 = t158 * t159;
            let t161 = t140 * t160;
            let t162 = f64x8::splat(1.0) * t161;
            let t163 = t43 * t1;
            let t166 = t163 * t143 * t133 * t54;
            let t167 = f64x8::splat(0.00018311447306006544) * t166;
            let t168 = t43 * t45;
            let t169 = t50 * t50;
            let t170 = f64x8::splat(1.0) / t169;
            let t175 = -f64x8::splat(0.8630833333333333) * t145 - f64x8::splat(0.301925) * t147 - f64x8::splat(0.05501625) * t151 - f64x8::splat(0.082785) * t156;
            let t177 = f64x8::splat(1.0) / t53;
            let t178 = t170 * t175 * t177;
            let t179 = t168 * t178;
            let t180 = f64x8::splat(0.5848223622634646) * t179;
            let t181 = t64 * v_rho;
            let t182 = f64x8::splat(1.0) / t181;
            let t188 = f64x8::splat(1.0) / t7 / t181;
            let t193 = f64x8::splat(1.0) / t13 / t24 / f64x8::splat(4.0);
            let t195 = t193 * t1 * t143;
            let t198 = -t62 * t182 * t68 * t71 / f64x8::splat(4.0) + t62 * t188 * t68 * t195 / f64x8::splat(32.0);
            let t199 = t75 * t198;
            let t200 = (simd::ln(t59));
            let t202 = t77 * t80;
            let t204 = t202 * t67 * t129;
            let t205 = t199 * t200 * t204;
            let t206 = t78 * t80;
            let t207 = t63 * v_rho;
            let t209 = f64x8::splat(1.0) / t7 / t207;
            let t214 = t77 * t77;
            let t215 = f64x8::splat(1.0) / t214;
            let t216 = param_beta * t215;
            let t217 = t79 * t79;
            let t218 = t216 * t217;
            let t219 = t100 * t100;
            let t220 = f64x8::splat(1.0) / t219;
            let t221 = t220 * t103;
            let t222 = t221 * t107;
            let t223 = t218 * t222;
            let t225 = f64x8::splat(1.0) / t110 / t67;
            let t226 = t108 * t225;
            let t227 = t226 * t1;
            let t228 = t113 * t6;
            let t229 = t137 + t162 - t167 - t180;
            let t230 = t229 * t99;
            let t231 = t228 * t230;
            let t232 = t227 * t231;
            let t236 = f64x8::splat(1.0) / t21 / t181;
            let t237 = t236 * t108;
            let t238 = t237 * t111;
            let t239 = t238 * t115;
            let t242 = -f64x8::splat(7.0) / f64x8::splat(288.0) * v_sigma * t209 * t39 * t90 + t223 * t232 / f64x8::splat(3072.0) - f64x8::splat(7.0) / f64x8::splat(4608.0) * t105 * t239;
            let t243 = param_beta * t242;
            let t245 = t120 * t93;
            let t246 = t124 * t124;
            let t247 = f64x8::splat(1.0) / t246;
            let t248 = t79 * t247;
            let t250 = t216 * t217 * t220;
            let t251 = t119 * t229;
            let t252 = t68 * t99;
            let t257 = t94 * t102 * t242 + t250 * t251 * t252;
            let t258 = t248 * t257;
            let t260 = t243 * t126 - t245 * t258;
            let t262 = f64x8::splat(1.0) / t128;
            let t264 = t206 * t67 * t260 * t262;
            let tvrho0 = -t32 + t57 + t131 + v_rho * (t137 + t162 - t167 - t180 + t205 + t264);
            acc_vrho = tvrho0;
            let t267 = t75 * param_alpha;
            let t268 = t60 * t65;
            let t271 = t202 * t129;
            let t272 = t71 * t200 * t271;
            let t274 = f64x8::splat(3.0) / f64x8::splat(32.0) * t267 * t268 * t272;
            let t278 = t18 * t88 * t5;
            let t282 = t94 * t102 * v_sigma;
            let t285 = t83 * t39 * t86 * t278 / f64x8::splat(96.0) + t282 * t116 / f64x8::splat(1536.0);
            let t286 = param_beta * t285;
            let t288 = param_beta * param_beta;
            let t289 = t288 * t119;
            let t290 = t289 * t215;
            let t291 = t217 * t247;
            let t293 = t291 * t101 * t285;
            let t295 = t286 * t126 - t290 * t293;
            let t296 = t67 * t295;
            let t298 = t206 * t296 * t262;
            let tvsigma0 = v_rho * (t274 + t298);
            acc_vsigma = tvsigma0;
            let t306 = t6 * t83;
            let t308 = t4 * t306 * t30;
            let t309 = f64x8::splat(0.0014764627977777779) * t308;
            let t310 = t4 * t6;
            let t311 = t133 * t139;
            let t313 = t310 * t311 * t160;
            let t314 = f64x8::splat(0.035616666666666665) * t313;
            let t315 = t138 * t26;
            let t316 = f64x8::splat(1.0) / t315;
            let t317 = t12 * t316;
            let t318 = t158 * t158;
            let t319 = t318 * t159;
            let t320 = t317 * t319;
            let t321 = f64x8::splat(2.0) * t320;
            let t322 = t71 * t18;
            let t323 = t19 * t5;
            let t325 = f64x8::splat(1.0) / t21 / t63;
            let t326 = t323 * t325;
            let t327 = t322 * t326;
            let t329 = t143 * t83;
            let t330 = t142 * t329;
            let t332 = t4 * t306;
            let t334 = f64x8::splat(1.0)/((t10).sqrt());
            let t335 = t334 * t18;
            let t336 = t335 * t326;
            let t338 = t150 * t329;
            let t341 = t20 * t5 * t325;
            let t343 = -f64x8::splat(0.4219833333333333) * t327 + f64x8::splat(0.8439666666666666) * t330 + f64x8::splat(0.3986222222222222) * t332 + f64x8::splat(0.06825833333333334) * t336 + f64x8::splat(0.13651666666666668) * t338 + f64x8::splat(0.1369277777777778) * t341;
            let t344 = t343 * t159;
            let t345 = t140 * t344;
            let t346 = f64x8::splat(1.0) * t345;
            let t347 = t138 * t138;
            let t348 = f64x8::splat(1.0) / t347;
            let t349 = t12 * t348;
            let t350 = t29 * t29;
            let t351 = f64x8::splat(1.0) / t350;
            let t352 = t318 * t351;
            let t353 = t349 * t352;
            let t354 = f64x8::splat(16.081979498692537) * t353;
            let t357 = t163 * t143 * t83 * t54;
            let t358 = f64x8::splat(0.00024415263074675396) * t357;
            let t359 = t43 * t4;
            let t361 = t359 * t134 * t178;
            let t362 = f64x8::splat(0.01084358130030174) * t361;
            let t363 = t169 * t50;
            let t364 = f64x8::splat(1.0) / t363;
            let t365 = t175 * t175;
            let t367 = t364 * t365 * t177;
            let t368 = t168 * t367;
            let t369 = f64x8::splat(1.1696447245269292) * t368;
            let t376 = -f64x8::splat(0.5753888888888888) * t327 + f64x8::splat(1.1507777777777777) * t330 + f64x8::splat(0.4025666666666667) * t332 + f64x8::splat(0.0366775) * t336 + f64x8::splat(0.073355) * t338 + f64x8::splat(0.137975) * t341;
            let t378 = t170 * t376 * t177;
            let t379 = t168 * t378;
            let t380 = f64x8::splat(0.5848223622634646) * t379;
            let t381 = t169 * t169;
            let t382 = f64x8::splat(1.0) / t381;
            let t383 = t382 * t365;
            let t384 = t53 * t53;
            let t385 = f64x8::splat(1.0) / t384;
            let t386 = t383 * t385;
            let t387 = t168 * t386;
            let t388 = f64x8::splat(17.315859105681465) * t387;
            let t389 = t198 * t198;
            let t390 = t75 * t389;
            let t391 = t200 * t200;
            let t393 = t390 * t391 * t204;
            let t394 = t64 * t63;
            let t395 = f64x8::splat(1.0) / t394;
            let t401 = f64x8::splat(1.0) / t7 / t394;
            let t407 = f64x8::splat(1.0) / t21 / t394;
            let t414 = f64x8::splat(1.0) / t13 / t2 * v_rho / f64x8::splat(48.0);
            let t416 = t414 * t18 * t323;
            let t419 = f64x8::splat(5.0) / f64x8::splat(4.0) * t62 * t395 * t68 * t71 - f64x8::splat(7.0) / f64x8::splat(24.0) * t62 * t401 * t68 * t195 + f64x8::splat(5.0) / f64x8::splat(48.0) * t62 * t407 * t68 * t416;
            let t420 = t75 * t419;
            let t422 = t420 * t200 * t204;
            let t423 = t200 * t77;
            let t424 = t199 * t423;
            let t425 = t260 * t262;
            let t426 = t81 * t425;
            let t427 = t424 * t426;
            let t430 = f64x8::splat(1.0) / t7 / t64;
            let t436 = f64x8::splat(1.0) / t214 / t77;
            let t437 = param_beta * t436;
            let t438 = t217 * t79;
            let t439 = t437 * t438;
            let t441 = f64x8::splat(1.0) / t219 / t100;
            let t442 = t441 * t103;
            let t443 = t442 * t107;
            let t444 = t439 * t443;
            let t445 = t110 * t110;
            let t447 = f64x8::splat(1.0) / t445 / t66;
            let t449 = t108 * t447 * t1;
            let t450 = t229 * t229;
            let t451 = t99 * t99;
            let t452 = t450 * t451;
            let t454 = t449 * t228 * t452;
            let t457 = t221 * t236;
            let t458 = t218 * t457;
            let t461 = -t309 - t314 - t321 + t346 + t354 + t358 + t362 + t369 - t380 - t388;
            let t462 = t461 * t99;
            let t464 = t227 * t228 * t462;
            let t467 = t439 * t222;
            let t468 = t450 * t99;
            let t470 = t449 * t228 * t468;
            let t475 = t407 * t108 * t111 * t115;
            let t478 = f64x8::splat(35.0) / f64x8::splat(432.0) * v_sigma * t430 * t39 * t90 + t444 * t454 / f64x8::splat(1536.0) - f64x8::splat(7.0) / f64x8::splat(2304.0) * t458 * t232 + t223 * t464 / f64x8::splat(3072.0) - t467 * t470 / f64x8::splat(3072.0) + f64x8::splat(119.0) / f64x8::splat(13824.0) * t105 * t475;
            let t479 = param_beta * t478;
            let t481 = t243 * t93;
            let t485 = f64x8::splat(1.0) / t246 / t124;
            let t486 = t79 * t485;
            let t487 = t257 * t257;
            let t488 = t486 * t487;
            let t492 = t437 * t438 * t441;
            let t493 = t119 * t450;
            let t495 = f64x8::splat(1.0) / t110 / t66;
            let t496 = t495 * t451;
            let t500 = t242 * t229;
            let t504 = t119 * t461;
            let t508 = t437 * t438 * t220;
            let t509 = t495 * t99;
            let t514 = t94 * t102 * t478 + f64x8::splat(2.0) * t250 * t500 * t252 + t250 * t504 * t252 + f64x8::splat(2.0) * t492 * t493 * t496 - t508 * t493 * t509;
            let t515 = t248 * t514;
            let t517 = t479 * t126 + f64x8::splat(2.0) * t245 * t488 - t245 * t515 - f64x8::splat(2.0) * t481 * t258;
            let t518 = t67 * t517;
            let t520 = t206 * t518 * t262;
            let t521 = t260 * t260;
            let t522 = t67 * t521;
            let t523 = t128 * t128;
            let t524 = f64x8::splat(1.0) / t523;
            let t526 = t206 * t522 * t524;
            let t527 = -t309 - t314 - t321 + t346 + t354 + t358 + t362 + t369 - t380 - t388 + t393 + t422 + f64x8::splat(2.0) * t427 + t520 - t526;
            let tv2rho20 = f64x8::splat(0.0022146941966666666) * t136 + f64x8::splat(2.0) * t161 - f64x8::splat(0.0003662289461201309) * t166 - f64x8::splat(1.1696447245269292) * t179 + f64x8::splat(2.0) * t205 + f64x8::splat(2.0) * t264 + v_rho * t527;
            acc_v2rho2 = tv2rho20;
            let t529 = t391 * param_alpha;
            let t530 = t529 * t60;
            let t531 = t199 * t530;
            let t532 = t65 * t71;
            let t533 = t532 * t271;
            let t534 = t531 * t533;
            let t536 = t60 * t182;
            let t538 = t267 * t536 * t272;
            let t540 = t267 * t60;
            let t541 = t188 * t193;
            let t542 = t541 * t200;
            let t543 = t540 * t542;
            let t544 = t271 * t310;
            let t545 = t543 * t544;
            let t548 = t267 * t268 * t71;
            let t550 = t80 * t260 * t262;
            let t551 = t423 * t550;
            let t552 = t548 * t551;
            let t554 = t295 * t262;
            let t555 = t81 * t554;
            let t556 = t424 * t555;
            let t561 = t220 * v_sigma;
            let t562 = t561 * t107;
            let t563 = t218 * t562;
            let t568 = -f64x8::splat(7.0) / f64x8::splat(288.0) * t209 * t39 * t86 * t278 + t563 * t232 / f64x8::splat(1536.0) - f64x8::splat(7.0) / f64x8::splat(2304.0) * t282 * t239;
            let t569 = param_beta * t568;
            let t571 = t286 * t93;
            let t573 = t288 * t242;
            let t574 = t573 * t215;
            let t576 = t215 * t217;
            let t577 = t289 * t576;
            let t578 = t485 * t101;
            let t579 = t285 * t257;
            let t580 = t578 * t579;
            let t583 = t436 * t438;
            let t584 = t583 * t247;
            let t585 = t289 * t584;
            let t586 = t220 * t285;
            let t588 = t229 * t68 * t99;
            let t589 = t586 * t588;
            let t592 = t291 * t101 * t568;
            let t594 = t569 * t126 - t571 * t258 - t290 * t592 - t574 * t293 + f64x8::splat(2.0) * t577 * t580 - t585 * t589;
            let t595 = t67 * t594;
            let t597 = t206 * t595 * t262;
            let t598 = t524 * t260;
            let t600 = t206 * t296 * t598;
            let tv2rhosigma0 = t274 + t298 + v_rho * (f64x8::splat(3.0) / f64x8::splat(32.0) * t534 - f64x8::splat(3.0) / f64x8::splat(8.0) * t538 + f64x8::splat(3.0) / f64x8::splat(64.0) * t545 + f64x8::splat(3.0) / f64x8::splat(32.0) * t552 + t556 + t597 - t600);
            acc_v2rhosigma = tv2rhosigma0;
            let t603 = param_alpha * param_alpha;
            let t604 = t75 * t603;
            let t605 = t64 * t207;
            let t606 = f64x8::splat(1.0) / t605;
            let t607 = v_sigma * t606;
            let t610 = t391 * t77;
            let t612 = t68 * t2 * t610 * t129;
            let t614 = f64x8::splat(3.0) / f64x8::splat(16384.0) * t604 * t607 * t612;
            let t615 = f64x8::splat(1.0) / t60;
            let t616 = t615 * t65;
            let t619 = f64x8::splat(3.0) / f64x8::splat(64.0) * t267 * t616 * t272;
            let t620 = t80 * t295;
            let t621 = t620 * t262;
            let t622 = t423 * t621;
            let t624 = f64x8::splat(3.0) / f64x8::splat(16.0) * t548 * t622;
            let t625 = t288 * t215;
            let t626 = t217 * t101;
            let t632 = t108 * t111 * t1 * t228 * t125;
            let t635 = t285 * t285;
            let t636 = t288 * t635;
            let t637 = t636 * t215;
            let t638 = t291 * t101;
            let t641 = t288 * param_beta;
            let t642 = t641 * t119;
            let t643 = t642 * t436;
            let t644 = t438 * t485;
            let t646 = t644 * t220 * t635;
            let t650 = t438 * t247 * t220;
            let t651 = t643 * t650;
            let t654 = t625 * t626 * t107 * t632 / f64x8::splat(1536.0) - f64x8::splat(2.0) * t637 * t638 + f64x8::splat(2.0) * t643 * t646 - t651 * t116 / f64x8::splat(1536.0);
            let t655 = t67 * t654;
            let t657 = t206 * t655 * t262;
            let t658 = t295 * t295;
            let t659 = t67 * t658;
            let t661 = t206 * t659 * t524;
            let tv2sigma20 = v_rho * (t614 + t619 + t624 + t657 - t661);
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
