//! GGA_X_PBETRANS lxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_pbetrans.c`
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

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_pbetrans_lxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    v3rho3: &mut [f64],
    v3rho2sigma: &mut [f64],
    v3rhosigma2: &mut [f64],
    v3sigma3: &mut [f64],
    v4rho4: &mut [f64],
    v4rho3sigma: &mut [f64],
    v4rho2sigma2: &mut [f64],
    v4rhosigma3: &mut [f64],
    v4sigma4: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
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
        let mut acc_v3rho3 = V_ZERO;
        let mut acc_v3rho2sigma = V_ZERO;
        let mut acc_v3rhosigma2 = V_ZERO;
        let mut acc_v3sigma3 = V_ZERO;
        let mut acc_v4rho4 = V_ZERO;
        let mut acc_v4rho3sigma = V_ZERO;
        let mut acc_v4rho2sigma2 = V_ZERO;
        let mut acc_v4rhosigma3 = V_ZERO;
        let mut acc_v4sigma4 = V_ZERO;
        {
            let t2 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t3 = f64x8::splat(M_CBRT3);
            let t4 = f64x8::splat(M_CBRTPI);
            let t6 = t3 / t4;
            let t7 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t8 = zeta_threshold - f64x8::splat(1.0);
            let t10 = ((t7).select(t8, (t7).select(-t8, f64x8::splat(0.0))));
            let t11 = f64x8::splat(1.0) + t10;
            let t13 = (simd::cbrt(zeta_threshold));
            let t15 = (simd::cbrt(t11));
            let t17 = (((t11).simd_le(zeta_threshold)).select(t13 * zeta_threshold, t15 * t11));
            let t18 = (simd::cbrt(v_rho));
            let t19 = t17 * t18;
            let t20 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t21 = (simd::cbrt(t20));
            let t23 = f64x8::splat(M_CBRT6);
            let t24 = t23 * t23;
            let t27 = ((v_sigma).sqrt());
            let t28 = f64x8::splat(M_CBRT2);
            let t29 = t27 * t28;
            let t31 = f64x8::splat(1.0) / t18 / v_rho;
            let t38 = (simd::exp(-f64x8::splat(2.0) * t3 * t21 * (t24 / t21 * t29 * t31 / f64x8::splat(12.0) - f64x8::splat(3.0))));
            let t39 = f64x8::splat(1.0) + t38;
            let t41 = f64x8::splat(0.413) / t39;
            let t42 = f64x8::splat(1.227) - t41;
            let t43 = t21 * t21;
            let t45 = t23 / t43;
            let t46 = t28 * t28;
            let t47 = v_sigma * t46;
            let t48 = v_rho * v_rho;
            let t49 = t18 * t18;
            let t51 = f64x8::splat(1.0) / t49 / t48;
            let t55 = f64x8::splat(1.227) - t41 + f64x8::splat(0.009125) * t45 * t47 * t51;
            let t56 = f64x8::splat(1.0) / t55;
            let t58 = -t42 * t56 + f64x8::splat(1.0);
            let t60 = t42 * t58 + f64x8::splat(1.0);
            let t64 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t60));
            let tzk0 = f64x8::splat(2.0) * t64;
            acc_zk = tzk0;
            let t66 = t17 / t49;
            let t70 = t39 * t39;
            let t71 = f64x8::splat(1.0) / t70;
            let t72 = t71 * t3;
            let t73 = t24 * t27;
            let t74 = t72 * t73;
            let t76 = f64x8::splat(1.0) / t18 / t48;
            let t77 = t28 * t76;
            let t78 = t38 * t58;
            let t79 = t77 * t78;
            let t82 = t38 * t56;
            let t83 = t77 * t82;
            let t86 = t55 * t55;
            let t87 = f64x8::splat(1.0) / t86;
            let t88 = t42 * t87;
            let t89 = t72 * t24;
            let t90 = t76 * t38;
            let t94 = t48 * v_rho;
            let t96 = f64x8::splat(1.0) / t49 / t94;
            let t100 = f64x8::splat(0.09177777777777778) * t89 * t29 * t90 - f64x8::splat(0.024333333333333332) * t45 * t47 * t96;
            let t102 = -f64x8::splat(0.09177777777777778) * t74 * t83 + t88 * t100;
            let t104 = f64x8::splat(0.09177777777777778) * t74 * t79 + t42 * t102;
            let t109 = ((t2).select(f64x8::splat(0.0), -t6 * t66 * t60 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t104));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t109 + f64x8::splat(2.0) * t64;
            acc_vrho = tvrho0;
            let t112 = f64x8::splat(1.0) / t27;
            let t113 = t24 * t112;
            let t114 = t72 * t113;
            let t115 = t28 * t31;
            let t116 = t115 * t78;
            let t119 = t115 * t82;
            let t122 = t112 * t28;
            let t123 = t31 * t38;
            let t127 = t46 * t51;
            let t130 = -f64x8::splat(0.034416666666666665) * t89 * t122 * t123 + f64x8::splat(0.009125) * t45 * t127;
            let t132 = f64x8::splat(0.034416666666666665) * t114 * t119 + t88 * t130;
            let t134 = -f64x8::splat(0.034416666666666665) * t114 * t116 + t42 * t132;
            let t138 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t134));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t138;
            acc_vsigma = tvsigma0;
            let t143 = t17 / t49 / v_rho;
            let t151 = f64x8::splat(1.0) / t70 / t39;
            let t152 = t3 * t3;
            let t153 = t151 * t152;
            let t154 = t23 * v_sigma;
            let t155 = t153 * t154;
            let t156 = t48 * t48;
            let t158 = f64x8::splat(1.0) / t49 / t156;
            let t159 = t46 * t158;
            let t160 = t38 * t38;
            let t161 = t160 * t58;
            let t162 = t159 * t161;
            let t166 = f64x8::splat(1.0) / t18 / t94;
            let t167 = t28 * t166;
            let t168 = t167 * t78;
            let t171 = t71 * t152;
            let t172 = t171 * t154;
            let t173 = t159 * t78;
            let t176 = t38 * t102;
            let t177 = t77 * t176;
            let t180 = t160 * t56;
            let t181 = t159 * t180;
            let t184 = t167 * t82;
            let t187 = t159 * t82;
            let t190 = t38 * t87;
            let t191 = t190 * t100;
            let t192 = t77 * t191;
            let t196 = f64x8::splat(1.0) / t86 / t55;
            let t197 = t42 * t196;
            let t198 = t100 * t100;
            let t201 = t153 * t23;
            let t202 = t158 * t160;
            let t206 = t166 * t38;
            let t210 = t171 * t23;
            let t211 = t158 * t38;
            let t218 = -f64x8::splat(0.24474074074074073) * t201 * t47 * t202 - f64x8::splat(0.21414814814814814) * t89 * t29 * t206 + f64x8::splat(0.12237037037037037) * t210 * t47 * t211 + f64x8::splat(0.08922222222222222) * t45 * t47 * t158;
            let t220 = f64x8::splat(0.24474074074074073) * t155 * t181 + f64x8::splat(0.21414814814814814) * t74 * t184 - f64x8::splat(0.12237037037037037) * t172 * t187 + f64x8::splat(0.18355555555555556) * t74 * t192 - f64x8::splat(2.0) * t197 * t198 + t88 * t218;
            let t222 = -f64x8::splat(0.24474074074074073) * t155 * t162 - f64x8::splat(0.21414814814814814) * t74 * t168 + f64x8::splat(0.12237037037037037) * t172 * t173 + f64x8::splat(0.18355555555555556) * t74 * t177 + t42 * t220;
            let t227 = ((t2).select(f64x8::splat(0.0), t6 * t143 * t60 / f64x8::splat(12.0) - t6 * t66 * t104 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t222));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t227 + f64x8::splat(4.0) * t109;
            acc_v2rho2 = tv2rho20;
            let t233 = t46 * t96;
            let t234 = t233 * t161;
            let t239 = t233 * t78;
            let t242 = t115 * t176;
            let t245 = t38 * t132;
            let t246 = t77 * t245;
            let t249 = t233 * t180;
            let t254 = t233 * t82;
            let t257 = t115 * t191;
            let t260 = t190 * t130;
            let t261 = t77 * t260;
            let t264 = t130 * t100;
            let t278 = f64x8::splat(0.09177777777777778) * t201 * t233 * t160 + f64x8::splat(0.04588888888888889) * t89 * t122 * t90 - f64x8::splat(0.04588888888888889) * t210 * t233 * t38 - f64x8::splat(0.024333333333333332) * t45 * t233;
            let t280 = -f64x8::splat(0.09177777777777778) * t201 * t249 - f64x8::splat(0.04588888888888889) * t114 * t83 + f64x8::splat(0.04588888888888889) * t210 * t254 - f64x8::splat(0.034416666666666665) * t114 * t257 + f64x8::splat(0.09177777777777778) * t74 * t261 - f64x8::splat(2.0) * t197 * t264 + t88 * t278;
            let t282 = f64x8::splat(0.09177777777777778) * t201 * t234 + f64x8::splat(0.04588888888888889) * t114 * t79 - f64x8::splat(0.04588888888888889) * t210 * t239 - f64x8::splat(0.034416666666666665) * t114 * t242 + f64x8::splat(0.09177777777777778) * t74 * t246 + t42 * t280;
            let t287 = ((t2).select(f64x8::splat(0.0), -t6 * t66 * t134 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t282));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t287 + f64x8::splat(2.0) * t138;
            acc_v2rhosigma = tv2rhosigma0;
            let t290 = f64x8::splat(1.0) / v_sigma;
            let t291 = t23 * t290;
            let t292 = t153 * t291;
            let t293 = t127 * t161;
            let t296 = t27 * v_sigma;
            let t297 = f64x8::splat(1.0) / t296;
            let t298 = t24 * t297;
            let t299 = t72 * t298;
            let t302 = t171 * t291;
            let t303 = t127 * t78;
            let t306 = t115 * t245;
            let t309 = t127 * t180;
            let t314 = t127 * t82;
            let t317 = t115 * t260;
            let t320 = t130 * t130;
            let t323 = t290 * t46;
            let t324 = t51 * t160;
            let t328 = t297 * t28;
            let t332 = t51 * t38;
            let t336 = -f64x8::splat(0.034416666666666665) * t201 * t323 * t324 + f64x8::splat(0.017208333333333332) * t89 * t328 * t123 + f64x8::splat(0.017208333333333332) * t210 * t323 * t332;
            let t338 = f64x8::splat(0.034416666666666665) * t292 * t309 - f64x8::splat(0.017208333333333332) * t299 * t119 - f64x8::splat(0.017208333333333332) * t302 * t314 - f64x8::splat(0.06883333333333333) * t114 * t317 - f64x8::splat(2.0) * t197 * t320 + t88 * t336;
            let t340 = -f64x8::splat(0.034416666666666665) * t292 * t293 + f64x8::splat(0.017208333333333332) * t299 * t116 + f64x8::splat(0.017208333333333332) * t302 * t303 - f64x8::splat(0.06883333333333333) * t114 * t306 + t42 * t338;
            let t344 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t340));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t344;
            acc_v2sigma2 = tv2sigma20;
            let t347 = t17 * t51;
            let t357 = t70 * t70;
            let t358 = f64x8::splat(1.0) / t357;
            let t359 = t358 * t296;
            let t360 = t156 * t94;
            let t361 = f64x8::splat(1.0) / t360;
            let t362 = t160 * t38;
            let t363 = t361 * t362;
            let t367 = t156 * v_rho;
            let t369 = f64x8::splat(1.0) / t49 / t367;
            let t370 = t46 * t369;
            let t371 = t370 * t161;
            let t374 = t151 * t296;
            let t375 = t361 * t160;
            let t379 = t160 * t102;
            let t380 = t159 * t379;
            let t384 = f64x8::splat(1.0) / t18 / t156;
            let t385 = t28 * t384;
            let t386 = t385 * t78;
            let t389 = t370 * t78;
            let t392 = t167 * t176;
            let t395 = t71 * t296;
            let t396 = t361 * t38;
            let t400 = t159 * t176;
            let t403 = t38 * t220;
            let t404 = t77 * t403;
            let t410 = t370 * t180;
            let t416 = t160 * t87;
            let t417 = t416 * t100;
            let t421 = t385 * t82;
            let t424 = t370 * t82;
            let t427 = t167 * t191;
            let t436 = t38 * t196;
            let t437 = t436 * t198;
            let t438 = t77 * t437;
            let t441 = t190 * t218;
            let t442 = t77 * t441;
            let t445 = t86 * t86;
            let t446 = f64x8::splat(1.0) / t445;
            let t447 = t42 * t446;
            let t448 = t198 * t100;
            let t462 = t384 * t38;
            let t475 = f64x8::splat(5.873777777777778) * t359 * t363 + f64x8::splat(1.7131851851851851) * t201 * t47 * t369 * t160 - f64x8::splat(5.873777777777778) * t374 * t375 + f64x8::splat(0.7138271604938271) * t89 * t29 * t462 - f64x8::splat(0.8565925925925926) * t210 * t47 * t369 * t38 + f64x8::splat(0.9789629629629629) * t395 * t396 - f64x8::splat(0.4163703703703704) * t45 * t47 * t369;
            let t477 = -f64x8::splat(5.873777777777778) * t359 * t363 * t56 - f64x8::splat(1.7131851851851851) * t155 * t410 + f64x8::splat(5.873777777777778) * t374 * t375 * t56 - f64x8::splat(0.7342222222222222) * t155 * t159 * t417 - f64x8::splat(0.7138271604938271) * t74 * t421 + f64x8::splat(0.8565925925925926) * t172 * t424 - f64x8::splat(0.6424444444444445) * t74 * t427 - f64x8::splat(0.9789629629629629) * t395 * t396 * t56 + f64x8::splat(0.3671111111111111) * t172 * t159 * t191 - f64x8::splat(0.5506666666666666) * t74 * t438 + f64x8::splat(0.2753333333333333) * t74 * t442 + f64x8::splat(6.0) * t447 * t448 - f64x8::splat(6.0) * t197 * t100 * t218 + t88 * t475;
            let t479 = f64x8::splat(5.873777777777778) * t359 * t363 * t58 + f64x8::splat(1.7131851851851851) * t155 * t371 - f64x8::splat(5.873777777777778) * t374 * t375 * t58 - f64x8::splat(0.7342222222222222) * t155 * t380 + f64x8::splat(0.7138271604938271) * t74 * t386 - f64x8::splat(0.8565925925925926) * t172 * t389 - f64x8::splat(0.6424444444444445) * t74 * t392 + f64x8::splat(0.9789629629629629) * t395 * t396 * t58 + f64x8::splat(0.3671111111111111) * t172 * t400 + f64x8::splat(0.2753333333333333) * t74 * t404 + t42 * t477;
            let t484 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(5.0) / f64x8::splat(36.0) * t6 * t347 * t60 + t6 * t143 * t104 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t66 * t222 - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t479));
            let tv3rho30 = f64x8::splat(2.0) * v_rho * t484 + f64x8::splat(6.0) * t227;
            acc_v3rho3 = tv3rho30;
            let t494 = t156 * t48;
            let t495 = f64x8::splat(1.0) / t494;
            let t496 = t358 * t495;
            let t497 = t362 * t58;
            let t498 = t497 * t27;
            let t503 = t151 * t495;
            let t504 = t161 * t27;
            let t507 = t233 * t379;
            let t516 = t71 * t495;
            let t517 = t27 * t38;
            let t518 = t517 * t58;
            let t521 = t233 * t176;
            let t524 = t115 * t403;
            let t527 = t160 * t132;
            let t528 = t159 * t527;
            let t531 = t167 * t245;
            let t534 = t159 * t245;
            let t537 = t38 * t280;
            let t538 = t77 * t537;
            let t541 = t190 * t278;
            let t542 = t77 * t541;
            let t545 = t416 * t130;
            let t556 = t115 * t441;
            let t559 = t362 * t56;
            let t560 = t559 * t27;
            let t563 = t180 * t27;
            let t566 = t517 * t56;
            let t572 = t23 * t46;
            let t573 = t171 * t572;
            let t574 = t96 * t38;
            let t575 = t87 * t100;
            let t581 = t153 * t572;
            let t582 = t96 * t160;
            let t586 = t167 * t260;
            let t590 = t72 * t73 * t28;
            let t591 = t196 * t130;
            let t592 = t591 * t100;
            let t593 = t90 * t592;
            let t596 = t115 * t437;
            let t607 = t362 * t27;
            let t613 = t160 * t27;
            let t626 = -f64x8::splat(2.2026666666666666) * t496 * t607 - f64x8::splat(0.4588888888888889) * t201 * t159 * t160 + f64x8::splat(2.2026666666666666) * t503 * t613 - f64x8::splat(0.10707407407407407) * t89 * t122 * t206 + f64x8::splat(0.22944444444444445) * t210 * t159 * t38 - f64x8::splat(0.3671111111111111) * t516 * t517 + f64x8::splat(0.08922222222222222) * t45 * t159;
            let t628 = f64x8::splat(0.18355555555555556) * t74 * t542 - f64x8::splat(0.24474074074074073) * t155 * t159 * t545 + f64x8::splat(0.12237037037037037) * t172 * t159 * t260 + f64x8::splat(0.10707407407407407) * t114 * t184 + f64x8::splat(0.09177777777777778) * t114 * t192 - f64x8::splat(0.034416666666666665) * t114 * t556 + f64x8::splat(2.2026666666666666) * t496 * t560 - f64x8::splat(2.2026666666666666) * t503 * t563 + f64x8::splat(0.3671111111111111) * t516 * t566 + f64x8::splat(6.0) * t447 * t130 * t198 - f64x8::splat(0.09177777777777778) * t573 * t574 * t575 + f64x8::splat(0.4588888888888889) * t201 * t181 + f64x8::splat(0.18355555555555556) * t581 * t582 * t575 - f64x8::splat(0.21414814814814814) * t74 * t586 - f64x8::splat(0.3671111111111111) * t590 * t593 + f64x8::splat(0.06883333333333333) * t114 * t596 - f64x8::splat(0.22944444444444445) * t210 * t187 - f64x8::splat(4.0) * t197 * t278 * t100 - f64x8::splat(2.0) * t197 * t130 * t218 + t88 * t626;
            let t630 = -f64x8::splat(2.2026666666666666) * t496 * t498 - f64x8::splat(0.4588888888888889) * t201 * t162 + f64x8::splat(2.2026666666666666) * t503 * t504 + f64x8::splat(0.18355555555555556) * t201 * t507 - f64x8::splat(0.10707407407407407) * t114 * t168 + f64x8::splat(0.22944444444444445) * t210 * t173 + f64x8::splat(0.09177777777777778) * t114 * t177 - f64x8::splat(0.3671111111111111) * t516 * t518 - f64x8::splat(0.09177777777777778) * t210 * t521 - f64x8::splat(0.034416666666666665) * t114 * t524 - f64x8::splat(0.24474074074074073) * t155 * t528 - f64x8::splat(0.21414814814814814) * t74 * t531 + f64x8::splat(0.12237037037037037) * t172 * t534 + f64x8::splat(0.18355555555555556) * t74 * t538 + t42 * t628;
            let t635 = ((t2).select(f64x8::splat(0.0), t6 * t143 * t134 / f64x8::splat(12.0) - t6 * t66 * t282 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t630));
            let tv3rho2sigma0 = f64x8::splat(2.0) * v_rho * t635 + f64x8::splat(4.0) * t287;
            acc_v3rho2sigma = tv3rho2sigma0;
            let t641 = t358 * t112;
            let t642 = f64x8::splat(1.0) / t367;
            let t643 = t642 * t362;
            let t649 = t151 * t112;
            let t650 = t642 * t160;
            let t651 = t650 * t58;
            let t654 = t127 * t379;
            let t663 = t71 * t112;
            let t664 = t642 * t38;
            let t668 = t127 * t176;
            let t671 = t233 * t527;
            let t676 = t233 * t245;
            let t679 = t115 * t537;
            let t682 = t38 * t338;
            let t683 = t77 * t682;
            let t691 = t650 * t56;
            let t694 = t127 * t417;
            let t706 = t127 * t191;
            let t709 = t87 * t130;
            let t719 = t72 * t113 * t28;
            let t720 = t123 * t592;
            let t723 = t115 * t541;
            let t726 = t436 * t320;
            let t727 = t77 * t726;
            let t736 = t190 * t336;
            let t737 = t77 * t736;
            let t758 = f64x8::splat(0.826) * t641 * t643 + f64x8::splat(0.04588888888888889) * t201 * t323 * t582 - f64x8::splat(0.826) * t649 * t650 - f64x8::splat(0.022944444444444444) * t89 * t328 * t90 - f64x8::splat(0.022944444444444444) * t210 * t323 * t574 + f64x8::splat(0.13766666666666666) * t663 * t664;
            let t760 = -f64x8::splat(0.826) * t641 * t643 * t56 - f64x8::splat(0.04588888888888889) * t292 * t249 + f64x8::splat(0.826) * t649 * t691 - f64x8::splat(0.034416666666666665) * t292 * t694 + f64x8::splat(0.022944444444444444) * t299 * t83 + f64x8::splat(0.022944444444444444) * t302 * t254 + f64x8::splat(0.017208333333333332) * t299 * t257 - f64x8::splat(0.13766666666666666) * t663 * t664 * t56 + f64x8::splat(0.017208333333333332) * t302 * t706 + f64x8::splat(0.18355555555555556) * t581 * t582 * t709 + f64x8::splat(0.09177777777777778) * t114 * t261 - f64x8::splat(0.09177777777777778) * t573 * t574 * t709 + f64x8::splat(0.13766666666666666) * t719 * t720 - f64x8::splat(0.06883333333333333) * t114 * t723 - f64x8::splat(0.18355555555555556) * t74 * t727 + f64x8::splat(6.0) * t447 * t320 * t100 - f64x8::splat(4.0) * t197 * t130 * t278 + f64x8::splat(0.09177777777777778) * t74 * t737 - f64x8::splat(2.0) * t197 * t336 * t100 + t88 * t758;
            let t762 = f64x8::splat(0.826) * t641 * t643 * t58 + f64x8::splat(0.04588888888888889) * t292 * t234 - f64x8::splat(0.826) * t649 * t651 - f64x8::splat(0.034416666666666665) * t292 * t654 - f64x8::splat(0.022944444444444444) * t299 * t79 - f64x8::splat(0.022944444444444444) * t302 * t239 + f64x8::splat(0.017208333333333332) * t299 * t242 + f64x8::splat(0.13766666666666666) * t663 * t664 * t58 + f64x8::splat(0.017208333333333332) * t302 * t668 + f64x8::splat(0.18355555555555556) * t201 * t671 + f64x8::splat(0.09177777777777778) * t114 * t246 - f64x8::splat(0.09177777777777778) * t210 * t676 - f64x8::splat(0.06883333333333333) * t114 * t679 + f64x8::splat(0.09177777777777778) * t74 * t683 + t42 * t760;
            let t767 = ((t2).select(f64x8::splat(0.0), -t6 * t66 * t340 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t762));
            let tv3rhosigma20 = f64x8::splat(2.0) * v_rho * t767 + f64x8::splat(2.0) * t344;
            acc_v3rhosigma2 = tv3rhosigma20;
            let t770 = t358 * t297;
            let t771 = f64x8::splat(1.0) / t156;
            let t772 = t771 * t362;
            let t773 = t772 * t58;
            let t776 = v_sigma * v_sigma;
            let t777 = f64x8::splat(1.0) / t776;
            let t778 = t23 * t777;
            let t779 = t153 * t778;
            let t782 = t151 * t297;
            let t783 = t771 * t160;
            let t784 = t783 * t58;
            let t787 = t127 * t527;
            let t791 = f64x8::splat(1.0) / t27 / t776;
            let t793 = t72 * t24 * t791;
            let t796 = t171 * t778;
            let t801 = t71 * t297;
            let t802 = t771 * t38;
            let t803 = t802 * t58;
            let t806 = t127 * t245;
            let t809 = t115 * t682;
            let t812 = t772 * t56;
            let t817 = t783 * t56;
            let t820 = t127 * t545;
            let t829 = t802 * t56;
            let t832 = t127 * t260;
            let t835 = t115 * t726;
            let t838 = t115 * t736;
            let t841 = t320 * t130;
            let t844 = t130 * t336;
            let t849 = t777 * t46;
            let t855 = t791 * t28;
            let t864 = -f64x8::splat(0.30975) * t770 * t772 + f64x8::splat(0.051625) * t201 * t849 * t324 + f64x8::splat(0.30975) * t782 * t783 - f64x8::splat(0.0258125) * t89 * t855 * t123 - f64x8::splat(0.0258125) * t210 * t849 * t332 - f64x8::splat(0.051625) * t801 * t802;
            let t866 = f64x8::splat(0.30975) * t770 * t812 - f64x8::splat(0.051625) * t779 * t309 - f64x8::splat(0.30975) * t782 * t817 - f64x8::splat(0.10325) * t292 * t820 + f64x8::splat(0.0258125) * t793 * t119 + f64x8::splat(0.0258125) * t796 * t314 + f64x8::splat(0.051625) * t299 * t317 + f64x8::splat(0.051625) * t801 * t829 + f64x8::splat(0.051625) * t302 * t832 + f64x8::splat(0.2065) * t114 * t835 - f64x8::splat(0.10325) * t114 * t838 + f64x8::splat(6.0) * t447 * t841 - f64x8::splat(6.0) * t197 * t844 + t88 * t864;
            let t868 = -f64x8::splat(0.30975) * t770 * t773 + f64x8::splat(0.051625) * t779 * t293 + f64x8::splat(0.30975) * t782 * t784 - f64x8::splat(0.10325) * t292 * t787 - f64x8::splat(0.0258125) * t793 * t116 - f64x8::splat(0.0258125) * t796 * t303 + f64x8::splat(0.051625) * t299 * t306 - f64x8::splat(0.051625) * t801 * t803 + f64x8::splat(0.051625) * t302 * t806 - f64x8::splat(0.10325) * t114 * t809 + t42 * t866;
            let t872 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t868));
            let tv3sigma30 = f64x8::splat(2.0) * v_rho * t872;
            acc_v3sigma3 = tv3sigma30;
            let t900 = t416 * t218;
            let t913 = t38 * t446;
            let t914 = t913 * t448;
            let t918 = t160 * t196;
            let t919 = t918 * t198;
            let t923 = t190 * t475;
            let t928 = f64x8::splat(1.0) / t49 / t494;
            let t929 = t46 * t928;
            let t934 = f64x8::splat(1.0) / t18 / t367;
            let t935 = t28 * t934;
            let t942 = t71 * t776;
            let t943 = t156 * t156;
            let t946 = f64x8::splat(1.0) / t18 / t943 / v_rho;
            let t948 = t942 * t946 * t3;
            let t949 = t24 * t28;
            let t950 = t949 * t82;
            let t953 = -f64x8::splat(3.4263703703703703) * t172 * t370 * t191 - f64x8::splat(1.284888888888889) * t74 * t167 * t441 + f64x8::splat(0.7342222222222222) * t172 * t159 * t441 + f64x8::splat(6.852740740740741) * t155 * t370 * t417 - f64x8::splat(1.4684444444444444) * t155 * t159 * t900 + f64x8::splat(2.8553086419753084) * t74 * t385 * t191 + f64x8::splat(2.569777777777778) * t74 * t167 * t437 - f64x8::splat(1.4684444444444444) * t172 * t159 * t437 + f64x8::splat(2.2026666666666666) * t74 * t77 * t914 + f64x8::splat(2.936888888888889) * t155 * t159 * t919 + f64x8::splat(0.3671111111111111) * t74 * t77 * t923 + f64x8::splat(11.611588477366254) * t155 * t929 * t180 + f64x8::splat(3.093251028806584) * t74 * t935 * t82 - f64x8::splat(5.805794238683127) * t172 * t929 * t82 - f64x8::splat(0.21754732510288066) * t948 * t950;
            let t955 = f64x8::splat(1.0) / t357 / t39;
            let t956 = t955 * t776;
            let t957 = t160 * t160;
            let t959 = t956 * t946 * t957;
            let t961 = t56 * t3 * t949;
            let t964 = t358 * t776;
            let t966 = t964 * t946 * t362;
            let t969 = t151 * t776;
            let t971 = t969 * t946 * t160;
            let t974 = f64x8::splat(1.0) / t943;
            let t975 = t974 * t38;
            let t979 = t974 * t160;
            let t983 = t974 * t362;
            let t989 = t957 * t3 * t949;
            let t996 = t362 * t3 * t949;
            let t1007 = t160 * t3 * t949;
            let t1021 = t3 * t24;
            let t1023 = t1021 * t28 * t38;
            let t1029 = -f64x8::splat(5.221135802469136) * t956 * t946 * t989 - f64x8::splat(82.2328888888889) * t359 * t983 + f64x8::splat(7.831703703703703) * t964 * t946 * t996 - f64x8::splat(11.611588477366254) * t201 * t47 * t928 * t160 + f64x8::splat(82.2328888888889) * t374 * t979 - f64x8::splat(3.045662551440329) * t969 * t946 * t1007 - f64x8::splat(3.093251028806584) * t89 * t29 * t934 * t38 + f64x8::splat(5.805794238683127) * t210 * t47 * t928 * t38 - f64x8::splat(13.705481481481481) * t395 * t975 + f64x8::splat(0.21754732510288066) * t942 * t946 * t1023 + f64x8::splat(2.3594320987654323) * t45 * t47 * t928;
            let t1033 = t42 / t445 / t55;
            let t1034 = t198 * t198;
            let t1037 = t218 * t218;
            let t1040 = t359 * t361;
            let t1041 = t362 * t87;
            let t1042 = t1041 * t100;
            let t1045 = t374 * t361;
            let t1048 = t395 * t361;
            let t1058 = t196 * t100 * t218;
            let t1062 = f64x8::splat(5.221135802469136) * t959 * t961 - f64x8::splat(7.831703703703703) * t966 * t961 + f64x8::splat(3.045662551440329) * t971 * t961 + f64x8::splat(13.705481481481481) * t395 * t975 * t56 - f64x8::splat(82.2328888888889) * t374 * t979 * t56 + f64x8::splat(82.2328888888889) * t359 * t983 * t56 + t88 * t1029 - f64x8::splat(24.0) * t1033 * t1034 - f64x8::splat(6.0) * t197 * t1037 + f64x8::splat(23.49511111111111) * t1040 * t1042 - f64x8::splat(23.49511111111111) * t1045 * t417 + f64x8::splat(3.9158518518518517) * t1048 * t191 + f64x8::splat(36.0) * t447 * t198 * t218 - f64x8::splat(8.0) * t197 * t100 * t475 - f64x8::splat(2.2026666666666666) * t590 * t90 * t1058;
            let t1068 = t38 * t477;
            let t1078 = t949 * t78;
            let t1082 = t58 * t3 * t949;
            let t1085 = t160 * t220;
            let t1127 = -f64x8::splat(5.221135802469136) * t959 * t1082 + f64x8::splat(7.831703703703703) * t966 * t1082 + f64x8::splat(6.852740740740741) * t155 * t370 * t379 - f64x8::splat(3.093251028806584) * t74 * t935 * t78 - f64x8::splat(11.611588477366254) * t155 * t929 * t161 + f64x8::splat(3.9158518518518517) * t395 * t396 * t102 + f64x8::splat(82.2328888888889) * t374 * t979 * t58 - f64x8::splat(23.49511111111111) * t374 * t375 * t102 - f64x8::splat(13.705481481481481) * t395 * t975 * t58 + f64x8::splat(23.49511111111111) * t359 * t363 * t102 - f64x8::splat(82.2328888888889) * t359 * t983 * t58;
            let t1133 = ((t2).select(f64x8::splat(0.0), f64x8::splat(10.0) / f64x8::splat(27.0) * t6 * t17 * t96 * t60 - f64x8::splat(5.0) / f64x8::splat(9.0) * t6 * t347 * t104 + t6 * t143 * t222 / f64x8::splat(2.0) - t6 * t66 * t479 / f64x8::splat(2.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * (t42 * (t953 + t1062) + f64x8::splat(0.7342222222222222) * t172 * t159 * t403 + f64x8::splat(0.3671111111111111) * t74 * t77 * t1068 - f64x8::splat(3.4263703703703703) * t172 * t370 * t176 - f64x8::splat(1.284888888888889) * t74 * t167 * t403 + f64x8::splat(0.21754732510288066) * t948 * t1078 - f64x8::splat(3.045662551440329) * t971 * t1082 - f64x8::splat(1.4684444444444444) * t155 * t159 * t1085 + f64x8::splat(2.8553086419753084) * t74 * t385 * t176 + f64x8::splat(5.805794238683127) * t172 * t929 * t78 + t1127)));
            let tv4rho40 = f64x8::splat(2.0) * v_rho * t1133 + f64x8::splat(8.0) * t484;
            acc_v4rho4 = tv4rho40;
            let t1164 = t496 * t362;
            let t1166 = t87 * t27 * t100;
            let t1169 = t503 * t160;
            let t1172 = t516 * t27;
            let t1175 = t1041 * t130;
            let t1193 = -f64x8::splat(2.4270123456790125) * t201 * t410 + f64x8::splat(1.2135061728395062) * t210 * t424 - f64x8::splat(6.608) * t1164 * t1166 + f64x8::splat(6.608) * t1169 * t1166 - f64x8::splat(1.1013333333333333) * t1172 * t191 + f64x8::splat(5.873777777777778) * t1040 * t1175 - f64x8::splat(5.873777777777778) * t1045 * t545 + f64x8::splat(0.9789629629629629) * t1048 * t260 + f64x8::splat(0.3671111111111111) * t172 * t159 * t541 - f64x8::splat(0.2065) * t114 * t115 * t914 + f64x8::splat(0.7138271604938271) * t74 * t385 * t260 - f64x8::splat(0.2753333333333333) * t114 * t438;
            let t1203 = t190 * t626;
            let t1214 = t416 * t278;
            let t1218 = t87 * t218;
            let t1226 = f64x8::splat(1.0) / t18 / t943;
            let t1227 = t955 * t1226;
            let t1230 = v_sigma * t3;
            let t1231 = t1230 * t949;
            let t1234 = t358 * t1226;
            let t1238 = -f64x8::splat(0.6424444444444445) * t74 * t167 * t541 + f64x8::splat(1.7131851851851851) * t155 * t370 * t545 - f64x8::splat(0.8565925925925926) * t172 * t370 * t260 + f64x8::splat(0.2753333333333333) * t74 * t77 * t1203 - f64x8::splat(0.32122222222222224) * t114 * t427 + f64x8::splat(0.13766666666666666) * t114 * t442 - f64x8::splat(0.034416666666666665) * t114 * t115 * t923 - f64x8::splat(0.7342222222222222) * t155 * t159 * t1214 + f64x8::splat(0.2753333333333333) * t581 * t582 * t1218 - f64x8::splat(0.13766666666666666) * t573 * t574 * t1218 - f64x8::splat(1.9579259259259258) * t1227 * t957 * t56 * t1231 + f64x8::splat(2.936888888888889) * t1234 * t559 * t1231;
            let t1240 = t151 * t1226;
            let t1244 = t71 * t1226;
            let t1245 = t1244 * t1230;
            let t1254 = t196 * t198;
            let t1266 = t358 * t361;
            let t1269 = t151 * t361;
            let t1272 = t71 * t361;
            let t1304 = f64x8::splat(1.9579259259259258) * t1227 * t957 * t1231 + f64x8::splat(24.229333333333333) * t1266 * t607 - f64x8::splat(2.936888888888889) * t1234 * t362 * t1231 + f64x8::splat(2.4270123456790125) * t201 * t370 * t160 - f64x8::splat(24.229333333333333) * t1269 * t613 + f64x8::splat(1.1421234567901235) * t1240 * t160 * t1231 + f64x8::splat(0.35691358024691355) * t89 * t122 * t462 - f64x8::splat(1.2135061728395062) * t210 * t370 * t38 + f64x8::splat(4.038222222222222) * t1272 * t517 - f64x8::splat(0.08158024691358025) * t1244 * v_sigma * t1023 - f64x8::splat(0.4163703703703704) * t45 * t370;
            let t1306 = -f64x8::splat(1.1421234567901235) * t1240 * t180 * t1231 + f64x8::splat(0.08158024691358025) * t1245 * t950 - f64x8::splat(1.3766666666666667) * t581 * t202 * t575 + f64x8::splat(0.6883333333333334) * t573 * t211 * t575 - f64x8::splat(0.5506666666666666) * t581 * t582 * t1254 + f64x8::splat(0.2753333333333333) * t573 * t574 * t1254 - f64x8::splat(0.35691358024691355) * t114 * t421 + f64x8::splat(18.0) * t447 * t264 * t218 - f64x8::splat(24.229333333333333) * t1266 * t560 + f64x8::splat(24.229333333333333) * t1269 * t563 - f64x8::splat(4.038222222222222) * t1272 * t566 + t88 * t1304;
            let t1317 = t196 * t278 * t100;
            let t1321 = t154 * t46;
            let t1331 = t446 * t130 * t198;
            let t1338 = t591 * t218;
            let t1351 = -f64x8::splat(24.0) * t1033 * t130 * t448 + f64x8::splat(18.0) * t447 * t278 * t198 - f64x8::splat(6.0) * t197 * t626 * t100 - f64x8::splat(1.1013333333333333) * t590 * t90 * t1317 + f64x8::splat(1.4684444444444444) * t153 * t1321 * t202 * t592 - f64x8::splat(0.7342222222222222) * t171 * t1321 * t211 * t592 + f64x8::splat(1.652) * t590 * t90 * t1331 + f64x8::splat(1.284888888888889) * t590 * t206 * t592 - f64x8::splat(0.5506666666666666) * t590 * t90 * t1338 + f64x8::splat(0.2065) * t719 * t123 * t1058 - f64x8::splat(6.0) * t197 * t278 * t218 - f64x8::splat(2.0) * t197 * t130 * t475;
            let t1355 = t160 * t280;
            let t1365 = t38 * t628;
            let t1379 = f64x8::splat(0.2753333333333333) * t201 * t233 * t1085 - f64x8::splat(1.2135061728395062) * t210 * t389 - f64x8::splat(1.3766666666666667) * t201 * t380 + f64x8::splat(2.4270123456790125) * t201 * t371 - f64x8::splat(0.13766666666666666) * t210 * t233 * t403 + f64x8::splat(0.6883333333333334) * t210 * t400 + t42 * (t1193 + t1238 + t1306 + t1351) - f64x8::splat(0.7342222222222222) * t155 * t159 * t1355 - f64x8::splat(0.6424444444444445) * t74 * t167 * t537 - f64x8::splat(0.034416666666666665) * t114 * t115 * t1068 + f64x8::splat(0.2753333333333333) * t74 * t77 * t1365 - f64x8::splat(0.8565925925925926) * t172 * t370 * t245 + f64x8::splat(0.3671111111111111) * t172 * t159 * t537 + f64x8::splat(0.13766666666666666) * t114 * t404 - f64x8::splat(0.08158024691358025) * t1245 * t1078;
            let t1425 = -f64x8::splat(0.32122222222222224) * t114 * t392 + f64x8::splat(1.7131851851851851) * t155 * t370 * t527 + f64x8::splat(0.7138271604938271) * t74 * t385 * t245 + f64x8::splat(0.35691358024691355) * t114 * t386 + f64x8::splat(1.9579259259259258) * t1227 * t957 * t58 * t1231 - f64x8::splat(2.936888888888889) * t1234 * t497 * t1231 + f64x8::splat(1.1421234567901235) * t1240 * t161 * t1231 + f64x8::splat(0.9789629629629629) * t395 * t396 * t132 + f64x8::splat(5.873777777777778) * t359 * t363 * t132 - f64x8::splat(5.873777777777778) * t374 * t375 * t132 + f64x8::splat(4.038222222222222) * t1272 * t518 - f64x8::splat(1.1013333333333333) * t516 * t517 * t102 - f64x8::splat(6.608) * t496 * t362 * t102 * t27 - f64x8::splat(24.229333333333333) * t1269 * t504 + f64x8::splat(6.608) * t503 * t379 * t27 + f64x8::splat(24.229333333333333) * t1266 * t498;
            let t1431 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(5.0) / f64x8::splat(36.0) * t6 * t347 * t134 + t6 * t143 * t282 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t66 * t630 - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * (t1379 + t1425)));
            let tv4rho3sigma0 = f64x8::splat(2.0) * v_rho * t1431 + f64x8::splat(6.0) * t635;
            acc_v4rho3sigma = tv4rho3sigma0;
            let t1445 = f64x8::splat(1.0) / t18 / t360;
            let t1447 = t151 * t1445 * t160;
            let t1451 = t955 * t1445 * t957;
            let t1455 = t358 * t1445 * t362;
            let t1464 = t71 * t1445 * t3;
            let t1474 = t495 * t38;
            let t1484 = t495 * t160;
            let t1491 = t495 * t362;
            let t1500 = f64x8::splat(0.3671111111111111) * t201 * t233 * t1355 - f64x8::splat(0.4282962962962963) * t1447 * t1082 - f64x8::splat(0.7342222222222222) * t1451 * t1082 + f64x8::splat(1.1013333333333333) * t1455 * t1082 + f64x8::splat(0.4588888888888889) * t210 * t534 - f64x8::splat(0.18355555555555556) * t210 * t233 * t537 + f64x8::splat(0.03059259259259259) * t1464 * t1078 - f64x8::splat(4.405333333333333) * t496 * t362 * t132 * t27 + f64x8::splat(4.405333333333333) * t503 * t527 * t27 - f64x8::splat(0.8718888888888889) * t663 * t1474 * t58 + f64x8::splat(0.2753333333333333) * t663 * t664 * t102 + f64x8::splat(1.652) * t641 * t643 * t102 + f64x8::splat(5.231333333333334) * t649 * t1484 * t58 - f64x8::splat(1.652) * t649 * t650 * t102 - f64x8::splat(5.231333333333334) * t641 * t1491 * t58 - f64x8::splat(0.7342222222222222) * t516 * t517 * t132 - f64x8::splat(0.9177777777777778) * t201 * t528;
            let t1524 = t918 * t320;
            let t1536 = f64x8::splat(0.7342222222222222) * t1451 * t961 - f64x8::splat(1.1013333333333333) * t1455 * t961 + f64x8::splat(0.4282962962962963) * t1447 * t961 - f64x8::splat(0.03059259259259259) * t1464 * t950 - f64x8::splat(0.7342222222222222) * t1172 * t260 + f64x8::splat(0.4282962962962963) * t74 * t167 * t726 - f64x8::splat(0.034416666666666665) * t302 * t127 * t437 - f64x8::splat(0.21414814814814814) * t114 * t586 + f64x8::splat(0.06883333333333333) * t292 * t127 * t919 - f64x8::splat(0.034416666666666665) * t299 * t596 + f64x8::splat(0.48948148148148146) * t155 * t159 * t1524 - f64x8::splat(0.24474074074074073) * t172 * t159 * t726 - f64x8::splat(0.06883333333333333) * t114 * t115 * t1203 + f64x8::splat(0.18355555555555556) * t114 * t542;
            let t1562 = t190 * t758;
            let t1566 = t416 * t336;
            let t1574 = t196 * t336 * t100;
            let t1581 = f64x8::splat(0.3671111111111111) * t573 * t574 * t592 + f64x8::splat(0.017208333333333332) * t299 * t556 + f64x8::splat(0.017208333333333332) * t302 * t127 * t441 - f64x8::splat(0.7342222222222222) * t581 * t582 * t592 - f64x8::splat(0.034416666666666665) * t292 * t127 * t900 - f64x8::splat(0.04588888888888889) * t299 * t192 - f64x8::splat(0.04588888888888889) * t302 * t233 * t191 + f64x8::splat(0.09177777777777778) * t292 * t233 * t417 + f64x8::splat(0.12237037037037037) * t172 * t159 * t736 + f64x8::splat(0.18355555555555556) * t74 * t77 * t1562 - f64x8::splat(0.24474074074074073) * t155 * t159 * t1566 - f64x8::splat(0.21414814814814814) * t74 * t167 * t736 - f64x8::splat(0.3671111111111111) * t590 * t90 * t1574 + f64x8::splat(0.2753333333333333) * t719 * t123 * t1317;
            let t1587 = t446 * t320 * t100;
            let t1591 = t591 * t278;
            let t1597 = t663 * t642;
            let t1600 = t709 * t27;
            let t1605 = t641 * t642;
            let t1608 = t649 * t642;
            let t1614 = t1021 * t28;
            let t1641 = t278 * t278;
            let t1644 = t87 * t278;
            let t1651 = f64x8::splat(0.13766666666666666) * t719 * t123 * t1338 + f64x8::splat(1.1013333333333333) * t590 * t90 * t1587 - f64x8::splat(0.7342222222222222) * t590 * t90 * t1591 - f64x8::splat(0.3671111111111111) * t719 * t593 + f64x8::splat(0.2753333333333333) * t1597 * t191 - f64x8::splat(4.405333333333333) * t1164 * t1600 + f64x8::splat(4.405333333333333) * t1169 * t1600 + f64x8::splat(1.652) * t1605 * t1042 - f64x8::splat(1.652) * t1608 * t417 - f64x8::splat(0.413) * t719 * t123 * t1331 + t88 * (-f64x8::splat(0.7342222222222222) * t1451 * t1614 - f64x8::splat(5.231333333333334) * t641 * t1491 + f64x8::splat(1.1013333333333333) * t1455 * t1614 - f64x8::splat(0.10707407407407407) * t201 * t323 * t202 + f64x8::splat(5.231333333333334) * t649 * t1484 - f64x8::splat(0.4282962962962963) * t1447 * t1614 + f64x8::splat(0.053537037037037036) * t89 * t328 * t206 + f64x8::splat(0.053537037037037036) * t210 * t323 * t211 - f64x8::splat(0.8718888888888889) * t663 * t1474 + f64x8::splat(0.03059259259259259) * t1464 * t949 * t38) - f64x8::splat(4.0) * t197 * t1641 + f64x8::splat(0.3671111111111111) * t581 * t582 * t1644 + f64x8::splat(0.4588888888888889) * t573 * t211 * t709;
            let t1694 = -f64x8::splat(0.18355555555555556) * t573 * t574 * t1644 - f64x8::splat(0.053537037037037036) * t302 * t187 - f64x8::splat(0.9177777777777778) * t581 * t202 * t709 - f64x8::splat(0.053537037037037036) * t299 * t184 + f64x8::splat(0.10707407407407407) * t292 * t181 + f64x8::splat(24.0) * t447 * t264 * t278 + f64x8::splat(5.231333333333334) * t641 * t1491 * t56 - f64x8::splat(5.231333333333334) * t649 * t1484 * t56 + f64x8::splat(0.8718888888888889) * t663 * t1474 * t56 - f64x8::splat(24.0) * t1033 * t320 * t198 + f64x8::splat(6.0) * t447 * t336 * t198 + f64x8::splat(6.0) * t447 * t320 * t218 - f64x8::splat(4.0) * t197 * t130 * t626 - f64x8::splat(4.0) * t197 * t758 * t100 - f64x8::splat(2.0) * t197 * t336 * t218;
            let t1698 = t160 * t338;
            let t1705 = t38 * t760;
            let t1739 = t42 * (t1536 + t1581 + t1651 + t1694) - f64x8::splat(0.24474074074074073) * t155 * t159 * t1698 + f64x8::splat(0.12237037037037037) * t172 * t159 * t682 + f64x8::splat(0.18355555555555556) * t74 * t77 * t1705 - f64x8::splat(0.21414814814814814) * t74 * t167 * t682 - f64x8::splat(0.21414814814814814) * t114 * t531 + f64x8::splat(0.053537037037037036) * t299 * t168 - f64x8::splat(0.10707407407407407) * t292 * t162 - f64x8::splat(0.06883333333333333) * t114 * t115 * t1365 + f64x8::splat(0.18355555555555556) * t114 * t538 + f64x8::splat(0.017208333333333332) * t302 * t127 * t403 + f64x8::splat(0.053537037037037036) * t302 * t173 - f64x8::splat(0.04588888888888889) * t302 * t521 + f64x8::splat(0.017208333333333332) * t299 * t524 + f64x8::splat(0.09177777777777778) * t292 * t507 - f64x8::splat(0.034416666666666665) * t292 * t127 * t1085 - f64x8::splat(0.04588888888888889) * t299 * t177;
            let t1745 = ((t2).select(f64x8::splat(0.0), t6 * t143 * t340 / f64x8::splat(12.0) - t6 * t66 * t762 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * (t1500 + t1739)));
            let tv4rho2sigma20 = f64x8::splat(2.0) * v_rho * t1745 + f64x8::splat(4.0) * t767;
            acc_v4rho2sigma2 = tv4rho2sigma20;
            let t1784 = t151 * t290;
            let t1786 = f64x8::splat(1.0) / t18 / t494;
            let t1788 = t1784 * t1786 * t160;
            let t1791 = -f64x8::splat(2.478) * t649 * t650 * t132 - f64x8::splat(0.051625) * t801 * t802 * t102 + f64x8::splat(0.413) * t663 * t664 * t132 - f64x8::splat(0.30975) * t770 * t772 * t102 + f64x8::splat(4e-20) * t782 * t651 + f64x8::splat(0.30975) * t782 * t783 * t102 + f64x8::splat(2.478) * t641 * t643 * t132 + f64x8::splat(0.13766666666666666) * t114 * t683 - f64x8::splat(0.06883333333333333) * t299 * t246 + f64x8::splat(0.034416666666666665) * t793 * t79 - f64x8::splat(0.06883333333333333) * t779 * t234 + f64x8::splat(0.13766666666666666) * t292 * t671 + f64x8::splat(0.051625) * t302 * t127 * t537 + f64x8::splat(0.16061111111111112) * t1788 * t1082;
            let t1801 = t955 * t290;
            let t1803 = t1801 * t1786 * t957;
            let t1806 = t358 * t290;
            let t1808 = t1806 * t1786 * t362;
            let t1820 = t71 * t290;
            let t1822 = t1820 * t1786 * t3;
            let t1825 = t38 * t866;
            let t1847 = t190 * t864;
            let t1851 = t913 * t841;
            let t1866 = -f64x8::splat(0.10325) * t292 * t127 * t1214 - f64x8::splat(0.2753333333333333) * t114 * t727 + f64x8::splat(0.13766666666666666) * t114 * t737 - f64x8::splat(0.06883333333333333) * t299 * t261 + f64x8::splat(0.13766666666666666) * t292 * t233 * t545 + f64x8::splat(0.09177777777777778) * t74 * t77 * t1847 + f64x8::splat(0.5506666666666666) * t74 * t77 * t1851 - f64x8::splat(0.06883333333333333) * t302 * t233 * t260 + f64x8::splat(0.051625) * t302 * t127 * t541 - f64x8::splat(0.10325) * t114 * t115 * t1562 - f64x8::splat(0.0258125) * t793 * t257;
            let t1875 = t770 * t771;
            let t1878 = t782 * t771;
            let t1885 = t801 * t771;
            let t1893 = t87 * t336;
            let t1897 = -f64x8::splat(0.0258125) * t796 * t706 + f64x8::splat(0.051625) * t299 * t723 + f64x8::splat(0.051625) * t779 * t694 + f64x8::splat(0.413) * t1597 * t260 - f64x8::splat(0.30975) * t1875 * t1042 + f64x8::splat(0.30975) * t1878 * t417 + f64x8::splat(2.478) * t1605 * t1175 - f64x8::splat(2.478) * t1608 * t545 - f64x8::splat(0.051625) * t1885 * t191 + f64x8::splat(18.0) * t447 * t844 * t100 - f64x8::splat(4e-20) * t782 * t691 + f64x8::splat(0.2753333333333333) * t581 * t582 * t1893;
            let t1904 = t196 * t320;
            let t1927 = -f64x8::splat(0.13766666666666666) * t573 * t574 * t1893 + f64x8::splat(0.011472222222222222) * t1822 * t950 - f64x8::splat(0.5506666666666666) * t581 * t582 * t1904 + f64x8::splat(0.2753333333333333) * t573 * t574 * t1904 - f64x8::splat(0.2753333333333333) * t1803 * t961 + f64x8::splat(0.413) * t1808 * t961 - f64x8::splat(0.16061111111111112) * t1788 * t961 - f64x8::splat(0.034416666666666665) * t796 * t254 + f64x8::splat(0.06883333333333333) * t779 * t249 - f64x8::splat(0.6195) * t719 * t123 * t1587 + f64x8::splat(0.413) * t719 * t123 * t1591;
            let t1931 = t591 * t336;
            let t1939 = t291 * t46;
            let t1988 = f64x8::splat(0.2065) * t719 * t123 * t1574 - f64x8::splat(0.5506666666666666) * t590 * t90 * t1931 - f64x8::splat(0.10325) * t72 * t298 * t28 * t720 - f64x8::splat(0.10325) * t171 * t1939 * t332 * t592 + f64x8::splat(0.2065) * t153 * t1939 * t324 * t592 + t88 * (f64x8::splat(0.2753333333333333) * t1801 * t1786 * t989 - f64x8::splat(0.413) * t1806 * t1786 * t996 - f64x8::splat(0.06883333333333333) * t201 * t849 * t582 + f64x8::splat(0.16061111111111112) * t1784 * t1786 * t1007 + f64x8::splat(0.034416666666666665) * t89 * t855 * t90 + f64x8::splat(0.034416666666666665) * t210 * t849 * t574 - f64x8::splat(0.011472222222222222) * t1820 * t1786 * t1023) - f64x8::splat(24.0) * t1033 * t841 * t100 + f64x8::splat(18.0) * t447 * t320 * t278 - f64x8::splat(6.0) * t197 * t278 * t336 - f64x8::splat(6.0) * t197 * t130 * t758 - f64x8::splat(2.0) * t197 * t864 * t100 - f64x8::splat(0.034416666666666665) * t793 * t83;
            let t1992 = -f64x8::splat(0.10325) * t292 * t127 * t1355 - f64x8::splat(0.0258125) * t793 * t242 + f64x8::splat(0.034416666666666665) * t796 * t239 - f64x8::splat(0.0258125) * t796 * t668 + f64x8::splat(0.2753333333333333) * t1803 * t1082 - f64x8::splat(0.413) * t1808 * t1082 + f64x8::splat(0.051625) * t779 * t654 - f64x8::splat(0.10325) * t114 * t115 * t1705 + f64x8::splat(0.051625) * t299 * t679 - f64x8::splat(0.06883333333333333) * t302 * t676 - f64x8::splat(0.011472222222222222) * t1822 * t1078 + f64x8::splat(0.09177777777777778) * t74 * t77 * t1825 + f64x8::splat(0.2753333333333333) * t201 * t233 * t1698 - f64x8::splat(0.13766666666666666) * t210 * t233 * t682 + t42 * (t1866 + t1897 + t1927 + t1988);
            let t1998 = ((t2).select(f64x8::splat(0.0), -t6 * t66 * t868 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * (t1791 + t1992)));
            let tv4rhosigma30 = f64x8::splat(2.0) * v_rho * t1998 + f64x8::splat(2.0) * t872;
            acc_v4rhosigma3 = tv4rhosigma30;
            let t2024 = t358 * t791;
            let t2027 = t151 * t791;
            let t2030 = t71 * t791;
            let t2033 = t71 * t777;
            let t2035 = t2033 * t934 * t3;
            let t2038 = t955 * t777;
            let t2040 = t2038 * t934 * t957;
            let t2043 = -f64x8::splat(0.13766666666666666) * t114 * t115 * t1847 + f64x8::splat(0.10325) * t302 * t127 * t736 - f64x8::splat(0.10325) * t793 * t317 - f64x8::splat(0.10325) * t796 * t832 + f64x8::splat(0.10325) * t299 * t838 + f64x8::splat(0.2065) * t779 * t820 - f64x8::splat(0.2065) * t292 * t127 * t1566 - f64x8::splat(0.2065) * t1885 * t260 - f64x8::splat(1.239) * t1875 * t1175 + f64x8::splat(1.239) * t1878 * t545 - f64x8::splat(0.92925) * t2024 * t812 + f64x8::splat(0.92925) * t2027 * t817 - f64x8::splat(0.154875) * t2030 * t829 - f64x8::splat(0.004302083333333333) * t2035 * t950 + f64x8::splat(0.10325) * t2040 * t961;
            let t2044 = t358 * t777;
            let t2046 = t2044 * t934 * t362;
            let t2049 = t151 * t777;
            let t2051 = t2049 * t934 * t160;
            let t2054 = t776 * v_sigma;
            let t2055 = f64x8::splat(1.0) / t2054;
            let t2056 = t23 * t2055;
            let t2057 = t171 * t2056;
            let t2061 = f64x8::splat(1.0) / t27 / t2054;
            let t2063 = t72 * t24 * t2061;
            let t2066 = t153 * t2056;
            let t2080 = t2055 * t46;
            let t2103 = t320 * t320;
            let t2106 = t336 * t336;
            let t2126 = -f64x8::splat(0.154875) * t2046 * t961 + f64x8::splat(0.06022916666666667) * t2051 * t961 - f64x8::splat(0.06453125) * t2057 * t314 - f64x8::splat(0.06453125) * t2063 * t119 + f64x8::splat(0.1290625) * t2066 * t309 + f64x8::splat(0.826) * t719 * t123 * t1931 + t88 * (-f64x8::splat(0.10325) * t2038 * t934 * t989 + f64x8::splat(0.92925) * t2024 * t772 + f64x8::splat(0.154875) * t2044 * t934 * t996 - f64x8::splat(0.1290625) * t201 * t2080 * t324 - f64x8::splat(0.92925) * t2027 * t783 - f64x8::splat(0.06022916666666667) * t2049 * t934 * t1007 + f64x8::splat(0.06453125) * t89 * t2061 * t28 * t123 + f64x8::splat(0.06453125) * t210 * t2080 * t332 + f64x8::splat(0.154875) * t2030 * t802 + f64x8::splat(0.004302083333333333) * t2033 * t934 * t1023) - f64x8::splat(24.0) * t1033 * t2103 - f64x8::splat(6.0) * t197 * t2106 - f64x8::splat(0.826) * t114 * t115 * t1851 - f64x8::splat(0.2065) * t299 * t835 - f64x8::splat(0.2065) * t302 * t127 * t726 + f64x8::splat(0.413) * t292 * t127 * t1524 + f64x8::splat(36.0) * t447 * t320 * t336 - f64x8::splat(8.0) * t197 * t130 * t864;
            let t2176 = f64x8::splat(0.2065) * t779 * t787 - f64x8::splat(0.06022916666666667) * t2051 * t1082 - f64x8::splat(0.2065) * t292 * t127 * t1698 + f64x8::splat(0.92925) * t2024 * t773 + f64x8::splat(0.154875) * t2030 * t803 - f64x8::splat(0.2065) * t801 * t802 * t132 - f64x8::splat(1.239) * t770 * t772 * t132 - f64x8::splat(0.92925) * t2027 * t784 + f64x8::splat(1.239) * t782 * t783 * t132 + f64x8::splat(0.06453125) * t2063 * t116 - f64x8::splat(0.1290625) * t2066 * t293;
            let t2181 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * (t42 * (t2043 + t2126) - f64x8::splat(0.13766666666666666) * t114 * t115 * t1825 + f64x8::splat(0.10325) * t299 * t809 + f64x8::splat(0.004302083333333333) * t2035 * t1078 + f64x8::splat(0.10325) * t302 * t127 * t682 - f64x8::splat(0.10325) * t793 * t306 + f64x8::splat(0.06453125) * t2057 * t303 - f64x8::splat(0.10325) * t796 * t806 - f64x8::splat(0.10325) * t2040 * t1082 + f64x8::splat(0.154875) * t2046 * t1082 + t2176)));
            let tv4sigma40 = f64x8::splat(2.0) * v_rho * t2181;
            acc_v4sigma4 = tv4sigma40;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rho2.into(); v2rho2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rhosigma.into(); v2rhosigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2sigma2.into(); v2sigma2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v3rho3.into(); v3rho3[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v3rho2sigma.into(); v3rho2sigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v3rhosigma2.into(); v3rhosigma2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v3sigma3.into(); v3sigma3[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v4rho4.into(); v4rho4[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v4rho3sigma.into(); v4rho3sigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v4rho2sigma2.into(); v4rho2sigma2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v4rhosigma3.into(); v4rhosigma3[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v4sigma4.into(); v4sigma4[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
