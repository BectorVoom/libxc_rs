//! MGGA_C_R2SCAN vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_r2scan.c`
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
pub fn mgga_c_r2scan_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    param_eta: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_eta = f64x8::splat(param_eta);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let v_lapl = load(lapl, ip, np);
        let v_tau = load(tau, ip, np);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_vsigma = V_ZERO;
        let mut acc_vlapl = V_ZERO;
        let mut acc_vtau = V_ZERO;
        {
            let t2 = f64x8::splat(M_CBRT3);
            let t3 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t4 = (simd::cbrt(t3));
            let t5 = t2 * t4;
            let t6 = f64x8::splat(M_CBRT4);
            let t7 = t6 * t6;
            let t8 = (simd::cbrt(v_rho));
            let t10 = t7 / t8;
            let t11 = t5 * t10;
            let t13 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t11;
            let t14 = ((t11).sqrt());
            let t16 = f64x8::splat(0.8969) * t11;
            let t17 = ((t11) * (t11).sqrt());
            let t18 = f64x8::splat(0.204775) * t17;
            let t19 = t2 * t2;
            let t20 = t4 * t4;
            let t21 = t19 * t20;
            let t22 = t8 * t8;
            let t25 = t21 * t6 / t22;
            let t26 = f64x8::splat(0.123235) * t25;
            let t27 = f64x8::splat(3.79785) * t14 + t16 + t18 + t26;
            let t30 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t27;
            let t31 = (simd::ln(t30));
            let t33 = f64x8::splat(0.0621814) * t13 * t31;
            let t34 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t35 = (simd::cbrt(zeta_threshold));
            let t37 = ((t34).select(t35 * zeta_threshold, f64x8::splat(1.0)));
            let t39 = f64x8::splat(2.0) * t37 - f64x8::splat(2.0);
            let t40 = f64x8::splat(M_CBRT2);
            let t41 = t40 - f64x8::splat(1.0);
            let t43 = f64x8::splat(1.0) / t41 / f64x8::splat(2.0);
            let t44 = t39 * t43;
            let t46 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t11;
            let t48 = f64x8::splat(0.905775) * t11;
            let t49 = f64x8::splat(0.1100325) * t17;
            let t50 = f64x8::splat(0.1241775) * t25;
            let t51 = f64x8::splat(5.1785) * t14 + t48 + t49 + t50;
            let t54 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t51;
            let t55 = (simd::ln(t54));
            let t58 = f64x8::splat(0.0197516734986138) * t44 * t46 * t55;
            let t59 = (simd::ln(f64x8::splat(2.0)));
            let t60 = f64x8::splat(1.0) - t59;
            let t61 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t63 = t60 / t61;
            let t64 = t35 * t35;
            let t65 = ((t34).select(t64, f64x8::splat(1.0)));
            let t66 = t65 * t65;
            let t67 = t66 * t65;
            let t69 = f64x8::splat(1.0) / t60;
            let t71 = f64x8::splat(1.0) / t67;
            let t72 = t61 * t71;
            let t74 = (simd::exp(-(-t33 + t58) * t69 * t72));
            let t75 = t74 - f64x8::splat(1.0);
            let t77 = f64x8::splat(1.0) + f64x8::splat(0.025) * t11;
            let t79 = f64x8::splat(1.0) + f64x8::splat(0.04445) * t11;
            let t80 = f64x8::splat(1.0) / t79;
            let t81 = t77 * t80;
            let t82 = v_rho * v_rho;
            let t84 = f64x8::splat(1.0) / t8 / t82;
            let t88 = f64x8::splat(1.0) / t66;
            let t90 = f64x8::splat(1.0) / t4;
            let t93 = f64x8::splat(1.0) / t75;
            let t94 = t6 * t69 * t93;
            let t95 = t88 * t19 * t90 * t94;
            let t99 = ((t34).select(t64 * zeta_threshold, f64x8::splat(1.0)));
            let t100 = f64x8::splat(1.0) / t99;
            let t101 = t69 * t100;
            let t102 = t71 * t93;
            let t103 = ((f64x8::splat(4.0)).sqrt());
            let t104 = t103 * t14;
            let t106 = f64x8::splat(0.03138525) * t11;
            let t107 = f64x8::splat(1.0) + f64x8::splat(0.022225) * t104 + t106;
            let t108 = t107 * t107;
            let t113 = f64x8::splat(1.0) - f64x8::splat(2.363) * t41 * t39 * t43;
            let t114 = f64x8::splat(1.0) / t108 * t113;
            let t115 = f64x8::splat(1.0) / t14;
            let t116 = t103 * t115;
            let t118 = f64x8::splat(0.04445) * t116 + f64x8::splat(0.125541);
            let t122 = f64x8::splat(1.898925) * t104 + t16 + t18 + t26;
            let t125 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t122;
            let t126 = (simd::ln(t125));
            let t128 = t122 * t122;
            let t129 = f64x8::splat(1.0) / t128;
            let t130 = t13 * t129;
            let t132 = ((t11).sqrt());
            let t135 = f64x8::splat(3.79785) * t116 + f64x8::splat(3.5876) + f64x8::splat(1.22865) * t132 + f64x8::splat(0.24647) * t11;
            let t136 = f64x8::splat(1.0) / t125;
            let t137 = t135 * t136;
            let t141 = f64x8::splat(2.58925) * t104 + t48 + t49 + t50;
            let t144 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t141;
            let t145 = (simd::ln(t144));
            let t148 = t44 * t46;
            let t149 = t141 * t141;
            let t150 = f64x8::splat(1.0) / t149;
            let t154 = f64x8::splat(5.1785) * t116 + f64x8::splat(3.6231) + f64x8::splat(0.660195) * t132 + f64x8::splat(0.248355) * t11;
            let t156 = f64x8::splat(1.0) / t144;
            let t157 = t150 * t154 * t156;
            let t160 = f64x8::splat(0.0285764) * t114 * t118 + f64x8::splat(0.01328816518) * t126 - f64x8::splat(1.0) * t130 * t137 - f64x8::splat(0.0021973736767207856) * t44 * t145 + f64x8::splat(0.5848223622634646) * t148 * t157;
            let t165 = f64x8::splat(1.0) + f64x8::splat(0.04445) * t14 + t106;
            let t166 = f64x8::splat(1.0) / t165;
            let t172 = f64x8::splat(5.0) * t5 * t10 * t160 - f64x8::splat(45.0) * param_eta * (-f64x8::splat(0.0285764) * t166 * t113 + t33 - t58);
            let t174 = t101 * t102 * t172;
            let t175 = f64x8::splat(M_CBRT6);
            let t176 = (simd::cbrt(t61));
            let t177 = t176 * t176;
            let t178 = f64x8::splat(1.0) / t177;
            let t179 = t175 * t178;
            let t180 = t40 * t40;
            let t181 = t179 * t180;
            let t183 = f64x8::splat(1.0) / t22 / t82;
            let t184 = v_sigma * t183;
            let t185 = t175 * t175;
            let t187 = f64x8::splat(1.0) / t176 / t61;
            let t188 = t185 * t187;
            let t189 = v_sigma * v_sigma;
            let t190 = t40 * t189;
            let t191 = t82 * t82;
            let t192 = t191 * v_rho;
            let t194 = f64x8::splat(1.0) / t8 / t192;
            let t198 = (simd::exp(-f64x8::splat(0.2044460407889637) * t188 * t190 * t194));
            let t200 = t181 * t184 * t198;
            let t203 = f64x8::splat(1.0) + f64x8::splat(0.027439371595564633) * t81 * v_sigma * t84 * t40 * t95 + f64x8::splat(0.043341108700271344) * t174 * t200;
            let t204 = ((t203).sqrt().sqrt());
            let t206 = f64x8::splat(1.0) - f64x8::splat(1.0) / t204;
            let t208 = t206 * t75 + f64x8::splat(1.0);
            let t209 = (simd::ln(t208));
            let t211 = t63 * t67 * t209;
            let t213 = f64x8::splat(1.0) / t22 / v_rho;
            let t216 = v_tau * t213 - t184 / f64x8::splat(8.0);
            let t220 = param_eta * v_sigma;
            let t223 = f64x8::splat(3.0) / f64x8::splat(20.0) * t185 * t177 * t40 + t220 * t183 / f64x8::splat(8.0);
            let t224 = f64x8::splat(1.0) / t223;
            let t225 = t216 * t224;
            let t226 = (t225).simd_le(f64x8::splat(0.0));
            let t227 = (f64x8::splat(0.0)).simd_lt(t225);
            let t228 = ((t227).select(f64x8::splat(0.0), t225));
            let t229 = f64x8::splat(1.0) - t228;
            let t230 = f64x8::splat(1.0) / t229;
            let t233 = (simd::exp(-f64x8::splat(0.64) * t228 * t230));
            let t234 = (t225).simd_le(f64x8::splat(2.5));
            let t235 = (f64x8::splat(2.5)).simd_lt(t225);
            let t236 = ((t235).select(f64x8::splat(2.5), t225));
            let t238 = t236 * t236;
            let t240 = t238 * t236;
            let t242 = t238 * t238;
            let t244 = t242 * t236;
            let t246 = t242 * t238;
            let t251 = ((t235).select(t225, f64x8::splat(2.5)));
            let t252 = f64x8::splat(1.0) - t251;
            let t255 = (simd::exp(f64x8::splat(1.5) / t252));
            let t257 = ((t226).select(t233, (t234).select(f64x8::splat(1.0) - f64x8::splat(0.64) * t236 - f64x8::splat(0.4352) * t238 - f64x8::splat(1.535685604549) * t240 + f64x8::splat(3.061560252175) * t242 - f64x8::splat(1.915710236206) * t244 + f64x8::splat(0.516884468372) * t246 - f64x8::splat(0.051848879792) * t242 * t240, -f64x8::splat(0.7) * t255)));
            let t260 = (simd::exp(f64x8::splat(1.0) * t166));
            let t261 = t260 - f64x8::splat(1.0);
            let t262 = t180 * v_sigma;
            let t263 = t262 * t183;
            let t266 = f64x8::splat(1.0) + f64x8::splat(0.02133764210437636) * t179 * t263;
            let t267 = ((t266).sqrt().sqrt());
            let t269 = f64x8::splat(1.0) - f64x8::splat(1.0) / t267;
            let t271 = t261 * t269 + f64x8::splat(1.0);
            let t272 = (simd::ln(t271));
            let t276 = (-f64x8::splat(0.0285764) * t166 + f64x8::splat(0.0285764) * t272) * t113 + t33 - t58 - t211;
            let t277 = t257 * t276;
            let tzk0 = -t33 + t58 + t211 + t277;
            acc_zk = tzk0;
            let t279 = f64x8::splat(1.0) / t8 / v_rho;
            let t280 = t7 * t279;
            let t282 = t5 * t280 * t31;
            let t283 = f64x8::splat(0.0011073470983333333) * t282;
            let t284 = t27 * t27;
            let t285 = f64x8::splat(1.0) / t284;
            let t286 = t13 * t285;
            let t287 = t115 * t2;
            let t288 = t4 * t7;
            let t289 = t288 * t279;
            let t290 = t287 * t289;
            let t292 = t5 * t280;
            let t293 = f64x8::splat(0.29896666666666666) * t292;
            let t294 = t132 * t2;
            let t295 = t294 * t289;
            let t296 = f64x8::splat(0.1023875) * t295;
            let t298 = t21 * t6 * t213;
            let t299 = f64x8::splat(0.08215666666666667) * t298;
            let t300 = -f64x8::splat(0.632975) * t290 - t293 - t296 - t299;
            let t301 = f64x8::splat(1.0) / t30;
            let t302 = t300 * t301;
            let t303 = t286 * t302;
            let t304 = f64x8::splat(1.0) * t303;
            let t305 = t44 * t2;
            let t308 = t305 * t288 * t279 * t55;
            let t309 = f64x8::splat(0.00018311447306006544) * t308;
            let t310 = t51 * t51;
            let t311 = f64x8::splat(1.0) / t310;
            let t313 = f64x8::splat(0.301925) * t292;
            let t314 = f64x8::splat(0.05501625) * t295;
            let t315 = f64x8::splat(0.082785) * t298;
            let t316 = -f64x8::splat(0.8630833333333333) * t290 - t313 - t314 - t315;
            let t318 = f64x8::splat(1.0) / t54;
            let t319 = t311 * t316 * t318;
            let t320 = t148 * t319;
            let t321 = f64x8::splat(0.5848223622634646) * t320;
            let t322 = t283 + t304 - t309 - t321;
            let t323 = t322 * t69;
            let t324 = t323 * t61;
            let t325 = t71 * t74;
            let t326 = t325 * t206;
            let t329 = f64x8::splat(1.0) / t204 / t203;
            let t330 = t75 * t329;
            let t331 = t82 * v_rho;
            let t333 = f64x8::splat(1.0) / t22 / t331;
            let t334 = t333 * t80;
            let t336 = t40 * t88;
            let t337 = t69 * t93;
            let t338 = t336 * t337;
            let t341 = t79 * t79;
            let t342 = f64x8::splat(1.0) / t341;
            let t343 = t77 * t342;
            let t344 = v_sigma * t333;
            let t349 = f64x8::splat(1.0) / t8 / t331;
            let t355 = t81 * v_sigma;
            let t356 = t84 * t40;
            let t357 = t66 * t66;
            let t359 = f64x8::splat(1.0) / t357 / t65;
            let t360 = t359 * t19;
            let t362 = t355 * t356 * t360;
            let t363 = t90 * t6;
            let t364 = t60 * t60;
            let t365 = f64x8::splat(1.0) / t364;
            let t366 = t363 * t365;
            let t367 = t75 * t75;
            let t368 = f64x8::splat(1.0) / t367;
            let t369 = t368 * t322;
            let t370 = t61 * t74;
            let t372 = t366 * t369 * t370;
            let t375 = t365 * t100;
            let t376 = t357 * t66;
            let t377 = f64x8::splat(1.0) / t376;
            let t378 = t375 * t377;
            let t379 = t368 * t172;
            let t380 = t379 * t179;
            let t381 = t378 * t380;
            let t382 = t198 * t322;
            let t383 = t382 * t370;
            let t384 = t263 * t383;
            let t392 = f64x8::splat(1.0) / t108 / t107 * t113;
            let t393 = (simd::pow(f64x8::splat(4.0), f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t394 = t393 * t115;
            let t395 = t5 * t279;
            let t396 = t394 * t395;
            let t398 = f64x8::splat(0.01046175) * t292;
            let t399 = -f64x8::splat(0.014816666666666667) * t396 - t398;
            let t400 = t118 * t399;
            let t403 = t114 * t393;
            let t405 = f64x8::splat(1.0) / t14 / t11;
            let t406 = t405 * t2;
            let t412 = -f64x8::splat(1.26595) * t396 - t293 - t296 - t299;
            let t416 = t5 * t7;
            let t417 = t279 * t129;
            let t421 = t128 * t122;
            let t422 = f64x8::splat(1.0) / t421;
            let t423 = t13 * t422;
            let t427 = t393 * t405;
            let t428 = t427 * t395;
            let t430 = f64x8::splat(1.0)/((t11).sqrt());
            let t431 = t430 * t2;
            let t432 = t431 * t289;
            let t435 = f64x8::splat(2.5319) * t428 - f64x8::splat(0.204775) * t432 - f64x8::splat(0.08215666666666667) * t292;
            let t436 = t435 * t136;
            let t439 = t128 * t128;
            let t440 = f64x8::splat(1.0) / t439;
            let t441 = t13 * t440;
            let t442 = t125 * t125;
            let t443 = f64x8::splat(1.0) / t442;
            let t444 = t135 * t443;
            let t449 = -f64x8::splat(1.7261666666666666) * t396 - t313 - t314 - t315;
            let t454 = t44 * t5;
            let t458 = t149 * t141;
            let t459 = f64x8::splat(1.0) / t458;
            let t460 = t459 * t154;
            let t461 = t156 * t449;
            let t468 = f64x8::splat(3.4523333333333333) * t428 - f64x8::splat(0.1100325) * t432 - f64x8::splat(0.082785) * t292;
            let t470 = t150 * t468 * t156;
            let t473 = t149 * t149;
            let t474 = f64x8::splat(1.0) / t473;
            let t475 = t474 * t154;
            let t476 = t144 * t144;
            let t477 = f64x8::splat(1.0) / t476;
            let t478 = t477 * t449;
            let t482 = -f64x8::splat(0.0571528) * t392 * t400 + f64x8::splat(0.0008468139866666666) * t403 * t406 * t4 * t279 - f64x8::splat(0.2137) * t129 * t412 * t136 + f64x8::splat(0.017808333333333332) * t416 * t417 * t137 + f64x8::splat(2.0) * t423 * t137 * t412 - f64x8::splat(1.0) * t130 * t436 - f64x8::splat(16.081979498692537) * t441 * t444 * t412 + f64x8::splat(0.06506148780181044) * t44 * t150 * t449 * t156 - f64x8::splat(0.00542179065015087) * t454 * t280 * t157 - f64x8::splat(1.1696447245269292) * t148 * t460 * t461 + f64x8::splat(0.5848223622634646) * t148 * t470 + f64x8::splat(17.315859105681465) * t148 * t475 * t478;
            let t486 = t165 * t165;
            let t487 = f64x8::splat(1.0) / t486;
            let t488 = t487 * t113;
            let t490 = -f64x8::splat(0.007408333333333334) * t290 - t398;
            let t496 = -f64x8::splat(5.0) / f64x8::splat(3.0) * t5 * t280 * t160 + f64x8::splat(5.0) * t5 * t10 * t482 - f64x8::splat(45.0) * param_eta * (f64x8::splat(0.0285764) * t488 * t490 - t283 - t304 + t309 + t321);
            let t498 = t101 * t102 * t496;
            let t502 = t181 * t344 * t198;
            let t505 = t101 * t102;
            let t506 = t189 * v_sigma;
            let t507 = t172 * t506;
            let t508 = t191 * t191;
            let t509 = t508 * v_rho;
            let t510 = f64x8::splat(1.0) / t509;
            let t511 = t510 * t198;
            let t515 = -f64x8::splat(0.002743937159556463) * t334 * v_sigma * t338 + f64x8::splat(0.004878720269691391) * t343 * t344 * t338 - f64x8::splat(0.0640252003896508) * t81 * v_sigma * t349 * t40 * t95 + f64x8::splat(0.027439371595564633) * t362 * t372 + f64x8::splat(0.043341108700271344) * t381 * t384 + f64x8::splat(0.043341108700271344) * t498 * t200 - f64x8::splat(0.11557628986739024) * t174 * t502 + f64x8::splat(0.005821825775391099) * t505 * t507 * t511;
            let t518 = -t324 * t326 + t330 * t515 / f64x8::splat(4.0);
            let t520 = f64x8::splat(1.0) / t208;
            let t522 = t63 * t67 * t518 * t520;
            let t526 = -f64x8::splat(5.0) / f64x8::splat(3.0) * v_tau * t183 + t344 / f64x8::splat(3.0);
            let t528 = t223 * t223;
            let t529 = f64x8::splat(1.0) / t528;
            let t530 = t216 * t529;
            let t531 = t220 * t333;
            let t534 = t526 * t224 + t530 * t531 / f64x8::splat(3.0);
            let t535 = ((t227).select(f64x8::splat(0.0), t534));
            let t538 = t229 * t229;
            let t539 = f64x8::splat(1.0) / t538;
            let t540 = t228 * t539;
            let t543 = -f64x8::splat(0.64) * t535 * t230 - f64x8::splat(0.64) * t540 * t535;
            let t544 = t543 * t233;
            let t545 = ((t235).select(f64x8::splat(0.0), t534));
            let t547 = t236 * t545;
            let t549 = t238 * t545;
            let t551 = t240 * t545;
            let t553 = t242 * t545;
            let t555 = t244 * t545;
            let t560 = t252 * t252;
            let t561 = f64x8::splat(1.0) / t560;
            let t562 = ((t235).select(t534, f64x8::splat(0.0)));
            let t566 = ((t226).select(t544, (t234).select(-f64x8::splat(0.64) * t545 - f64x8::splat(0.8704) * t547 - f64x8::splat(4.607056813647) * t549 + f64x8::splat(12.2462410087) * t551 - f64x8::splat(9.57855118103) * t553 + f64x8::splat(3.101306810232) * t555 - f64x8::splat(0.362942158544) * t246 * t545, -f64x8::splat(1.05) * t561 * t562 * t255)));
            let t567 = t566 * t276;
            let t568 = t487 * t490;
            let t570 = t260 * t269;
            let t574 = f64x8::splat(1.0) / t267 / t266;
            let t575 = t261 * t574;
            let t576 = t575 * t175;
            let t577 = t178 * t180;
            let t581 = -f64x8::splat(1.0) * t568 * t570 - f64x8::splat(0.014225094736250906) * t576 * t577 * t344;
            let t582 = f64x8::splat(1.0) / t271;
            let t587 = (f64x8::splat(0.0285764) * t568 + f64x8::splat(0.0285764) * t581 * t582) * t113 - t283 - t304 + t309 + t321 - t522;
            let t588 = t257 * t587;
            let tvrho0 = -t33 + t58 + t211 + t277 + v_rho * (t283 + t304 - t309 - t321 + t522 + t567 + t588);
            acc_vrho = tvrho0;
            let t591 = t63 * t67;
            let t594 = t19 * t90;
            let t595 = t594 * t94;
            let t598 = t180 * t183;
            let t599 = t598 * t198;
            let t600 = t179 * t599;
            let t604 = f64x8::splat(1.0) / t508;
            let t605 = t604 * t198;
            let t609 = f64x8::splat(0.027439371595564633) * t81 * t356 * t88 * t595 + f64x8::splat(0.043341108700271344) * t174 * t600 - f64x8::splat(0.002183184665771662) * t505 * t172 * t189 * t605;
            let t610 = t609 * t520;
            let t613 = t591 * t330 * t610 / f64x8::splat(4.0);
            let t614 = t183 * t224;
            let t615 = param_eta * t183;
            let t618 = -t530 * t615 / f64x8::splat(8.0) - t614 / f64x8::splat(8.0);
            let t619 = ((t227).select(f64x8::splat(0.0), t618));
            let t624 = -f64x8::splat(0.64) * t619 * t230 - f64x8::splat(0.64) * t540 * t619;
            let t625 = t624 * t233;
            let t626 = ((t235).select(f64x8::splat(0.0), t618));
            let t628 = t236 * t626;
            let t630 = t238 * t626;
            let t632 = t240 * t626;
            let t634 = t242 * t626;
            let t636 = t244 * t626;
            let t641 = ((t235).select(t618, f64x8::splat(0.0)));
            let t645 = ((t226).select(t625, (t234).select(-f64x8::splat(0.64) * t626 - f64x8::splat(0.8704) * t628 - f64x8::splat(4.607056813647) * t630 + f64x8::splat(12.2462410087) * t632 - f64x8::splat(9.57855118103) * t634 + f64x8::splat(3.101306810232) * t636 - f64x8::splat(0.362942158544) * t246 * t626, -f64x8::splat(1.05) * t561 * t641 * t255)));
            let t646 = t645 * t276;
            let t647 = t575 * t179;
            let t648 = t582 * t113;
            let t652 = f64x8::splat(0.00015243824895787514) * t647 * t598 * t648 - t613;
            let t653 = t257 * t652;
            let tvsigma0 = v_rho * (t613 + t646 + t653);
            acc_vsigma = tvsigma0;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl = tvlapl0;
            let t655 = t213 * t224;
            let t656 = ((t227).select(f64x8::splat(0.0), t655));
            let t661 = -f64x8::splat(0.64) * t656 * t230 - f64x8::splat(0.64) * t540 * t656;
            let t662 = t661 * t233;
            let t663 = ((t235).select(f64x8::splat(0.0), t655));
            let t665 = t236 * t663;
            let t667 = t238 * t663;
            let t669 = t240 * t663;
            let t671 = t242 * t663;
            let t673 = t244 * t663;
            let t678 = ((t235).select(t655, f64x8::splat(0.0)));
            let t682 = ((t226).select(t662, (t234).select(-f64x8::splat(0.64) * t663 - f64x8::splat(0.8704) * t665 - f64x8::splat(4.607056813647) * t667 + f64x8::splat(12.2462410087) * t669 - f64x8::splat(9.57855118103) * t671 + f64x8::splat(3.101306810232) * t673 - f64x8::splat(0.362942158544) * t246 * t663, -f64x8::splat(1.05) * t561 * t678 * t255)));
            let t683 = v_rho * t682;
            let tvtau0 = t683 * t276;
            acc_vtau = tvtau0;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        store_add(vlapl, ip, m, acc_vlapl);
        store_add(vtau, ip, m, acc_vtau);
        ip += 8;
    }
}
