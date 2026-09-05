//! GGA_C_SOGGA11 fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_sogga11.c`
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
pub fn gga_c_sogga11_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    param_sogga11_a_1: f64,
    param_sogga11_a_2: f64,
    param_sogga11_a_3: f64,
    param_sogga11_a_4: f64,
    param_sogga11_a_5: f64,
    param_sogga11_b_1: f64,
    param_sogga11_b_2: f64,
    param_sogga11_b_3: f64,
    param_sogga11_b_4: f64,
    param_sogga11_b_5: f64,
    param_sogga11_a_0: f64,
    param_sogga11_b_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_sogga11_a_1 = f64x8::splat(param_sogga11_a_1);
    let param_sogga11_a_2 = f64x8::splat(param_sogga11_a_2);
    let param_sogga11_a_3 = f64x8::splat(param_sogga11_a_3);
    let param_sogga11_a_4 = f64x8::splat(param_sogga11_a_4);
    let param_sogga11_a_5 = f64x8::splat(param_sogga11_a_5);
    let param_sogga11_b_1 = f64x8::splat(param_sogga11_b_1);
    let param_sogga11_b_2 = f64x8::splat(param_sogga11_b_2);
    let param_sogga11_b_3 = f64x8::splat(param_sogga11_b_3);
    let param_sogga11_b_4 = f64x8::splat(param_sogga11_b_4);
    let param_sogga11_b_5 = f64x8::splat(param_sogga11_b_5);
    let param_sogga11_a_0 = f64x8::splat(param_sogga11_a_0);
    let param_sogga11_b_0 = f64x8::splat(param_sogga11_b_0);
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
            let t33 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t34 = (simd::cbrt(zeta_threshold));
            let t36 = ((t33).select(t34 * zeta_threshold, f64x8::splat(1.0)));
            let t39 = f64x8::splat(M_CBRT2);
            let t43 = (f64x8::splat(2.0) * t36 - f64x8::splat(2.0)) / (f64x8::splat(2.0) * t39 - f64x8::splat(2.0));
            let t45 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t10;
            let t50 = f64x8::splat(5.1785) * t13 + f64x8::splat(0.905775) * t10 + f64x8::splat(0.1100325) * t16 + f64x8::splat(0.1241775) * t24;
            let t53 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t50;
            let t54 = (simd::ln(t53));
            let t58 = -f64x8::splat(0.0621814) * t12 * t30 + f64x8::splat(0.0197516734986138) * t43 * t45 * t54;
            let t60 = param_sogga11_a_1;
            let t61 = t34 * t34;
            let t62 = ((t33).select(t61, f64x8::splat(1.0)));
            let t63 = t39 * t62;
            let t64 = v_rho * v_rho;
            let t66 = f64x8::splat(1.0) / t7 / t64;
            let t67 = v_sigma * t66;
            let t68 = t63 * t67;
            let t69 = f64x8::splat(1.0) / t3;
            let t70 = t18 * t69;
            let t71 = f64x8::splat(1.0) / t58;
            let t72 = t5 * t71;
            let t73 = t70 * t72;
            let t75 = f64x8::splat(0.0006950658458333333) * t68 * t73;
            let t76 = f64x8::splat(1.0) - t75;
            let t78 = f64x8::splat(1.0) - f64x8::splat(1.0) / t76;
            let t80 = param_sogga11_a_2;
            let t81 = t78 * t78;
            let t83 = param_sogga11_a_3;
            let t84 = t81 * t78;
            let t86 = param_sogga11_a_4;
            let t87 = t81 * t81;
            let t89 = param_sogga11_a_5;
            let t93 = param_sogga11_b_1;
            let t94 = (simd::exp(t75));
            let t95 = f64x8::splat(1.0) - t94;
            let t97 = param_sogga11_b_2;
            let t98 = t95 * t95;
            let t100 = param_sogga11_b_3;
            let t101 = t98 * t95;
            let t103 = param_sogga11_b_4;
            let t104 = t98 * t98;
            let t106 = param_sogga11_b_5;
            let t109 = t106 * t104 * t95 + t89 * t87 * t78 + t100 * t101 + t103 * t104 + t60 * t78 + t80 * t81 + t83 * t84 + t86 * t87 + t93 * t95 + t97 * t98 + param_sogga11_a_0 + param_sogga11_b_0;
            let tzk0 = t58 * t109;
            acc_zk = tzk0;
            let t111 = f64x8::splat(1.0) / t7 / v_rho;
            let t112 = t6 * t111;
            let t116 = t26 * t26;
            let t117 = f64x8::splat(1.0) / t116;
            let t118 = t12 * t117;
            let t120 = f64x8::splat(1.0) / t13 * t1;
            let t121 = t3 * t6;
            let t122 = t121 * t111;
            let t123 = t120 * t122;
            let t125 = t4 * t112;
            let t127 = ((t10).sqrt());
            let t128 = t127 * t1;
            let t129 = t128 * t122;
            let t134 = t20 * t5 / t21 / v_rho;
            let t136 = -f64x8::splat(0.632975) * t123 - f64x8::splat(0.29896666666666666) * t125 - f64x8::splat(0.1023875) * t129 - f64x8::splat(0.08215666666666667) * t134;
            let t137 = f64x8::splat(1.0) / t29;
            let t138 = t136 * t137;
            let t141 = t43 * t1;
            let t146 = t43 * t45;
            let t147 = t50 * t50;
            let t148 = f64x8::splat(1.0) / t147;
            let t153 = -f64x8::splat(0.8630833333333333) * t123 - f64x8::splat(0.301925) * t125 - f64x8::splat(0.05501625) * t129 - f64x8::splat(0.082785) * t134;
            let t155 = f64x8::splat(1.0) / t53;
            let t156 = t148 * t153 * t155;
            let t159 = f64x8::splat(0.0011073470983333333) * t4 * t112 * t30 + f64x8::splat(1.0) * t118 * t138 - f64x8::splat(0.00018311447306006544) * t141 * t121 * t111 * t54 - f64x8::splat(0.5848223622634646) * t146 * t156;
            let t160 = v_rho * t159;
            let t162 = v_rho * t58;
            let t163 = t76 * t76;
            let t164 = f64x8::splat(1.0) / t163;
            let t165 = t60 * t164;
            let t166 = t64 * v_rho;
            let t168 = f64x8::splat(1.0) / t7 / t166;
            let t169 = v_sigma * t168;
            let t170 = t63 * t169;
            let t173 = t58 * t58;
            let t174 = f64x8::splat(1.0) / t173;
            let t175 = t5 * t174;
            let t176 = t175 * t159;
            let t177 = t70 * t176;
            let t180 = f64x8::splat(0.0016218203069444444) * t170 * t73 + f64x8::splat(0.0006950658458333333) * t68 * t177;
            let t182 = t80 * t78;
            let t183 = t164 * t180;
            let t186 = t83 * t81;
            let t189 = t86 * t84;
            let t192 = t89 * t87;
            let t195 = -t180;
            let t196 = t93 * t195;
            let t198 = t97 * t95;
            let t199 = t195 * t94;
            let t202 = t100 * t98;
            let t205 = t103 * t101;
            let t208 = t106 * t104;
            let t211 = t165 * t180 + f64x8::splat(2.0) * t182 * t183 + f64x8::splat(3.0) * t186 * t183 + f64x8::splat(4.0) * t189 * t183 + f64x8::splat(5.0) * t192 * t183 - t196 * t94 - f64x8::splat(2.0) * t198 * t199 - f64x8::splat(3.0) * t202 * t199 - f64x8::splat(4.0) * t205 * t199 - f64x8::splat(5.0) * t208 * t199;
            let tvrho0 = t160 * t109 + t162 * t211 + tzk0;
            acc_vrho = tvrho0;
            let t213 = t165 * t63;
            let t214 = t66 * t18;
            let t215 = t69 * t5;
            let t216 = t215 * t71;
            let t217 = t214 * t216;
            let t221 = t164 * t39 * t62;
            let t222 = t182 * t221;
            let t225 = t186 * t221;
            let t228 = t189 * t221;
            let t231 = t192 * t221;
            let t234 = t93 * t39;
            let t235 = t62 * t66;
            let t238 = t70 * t72 * t94;
            let t241 = t63 * t66;
            let t242 = t198 * t241;
            let t245 = t202 * t241;
            let t248 = t205 * t241;
            let t251 = t208 * t241;
            let t254 = -f64x8::splat(0.0006950658458333333) * t213 * t217 - f64x8::splat(0.0013901316916666666) * t222 * t217 - f64x8::splat(0.0020851975375) * t225 * t217 - f64x8::splat(0.0027802633833333332) * t228 * t217 - f64x8::splat(0.0034753292291666666) * t231 * t217 - f64x8::splat(0.0006950658458333333) * t234 * t235 * t238 - f64x8::splat(0.0013901316916666666) * t242 * t238 - f64x8::splat(0.0020851975375) * t245 * t238 - f64x8::splat(0.0027802633833333332) * t248 * t238 - f64x8::splat(0.0034753292291666666) * t251 * t238;
            let tvsigma0 = t162 * t254;
            acc_vsigma = tvsigma0;
            let t259 = t6 * t66;
            let t263 = t4 * t6;
            let t264 = t111 * t117;
            let t268 = t116 * t26;
            let t269 = f64x8::splat(1.0) / t268;
            let t270 = t12 * t269;
            let t271 = t136 * t136;
            let t272 = t271 * t137;
            let t277 = f64x8::splat(1.0) / t13 / t10 * t18;
            let t278 = t19 * t5;
            let t280 = f64x8::splat(1.0) / t21 / t64;
            let t281 = t278 * t280;
            let t282 = t277 * t281;
            let t284 = t121 * t66;
            let t285 = t120 * t284;
            let t287 = t4 * t259;
            let t289 = f64x8::splat(1.0)/((t10).sqrt());
            let t290 = t289 * t18;
            let t291 = t290 * t281;
            let t293 = t128 * t284;
            let t296 = t20 * t5 * t280;
            let t298 = -f64x8::splat(0.4219833333333333) * t282 + f64x8::splat(0.8439666666666666) * t285 + f64x8::splat(0.3986222222222222) * t287 + f64x8::splat(0.06825833333333334) * t291 + f64x8::splat(0.13651666666666668) * t293 + f64x8::splat(0.1369277777777778) * t296;
            let t299 = t298 * t137;
            let t302 = t116 * t116;
            let t303 = f64x8::splat(1.0) / t302;
            let t304 = t12 * t303;
            let t305 = t29 * t29;
            let t306 = f64x8::splat(1.0) / t305;
            let t307 = t271 * t306;
            let t314 = t43 * t4;
            let t318 = t147 * t50;
            let t319 = f64x8::splat(1.0) / t318;
            let t320 = t153 * t153;
            let t322 = t319 * t320 * t155;
            let t331 = -f64x8::splat(0.5753888888888888) * t282 + f64x8::splat(1.1507777777777777) * t285 + f64x8::splat(0.4025666666666667) * t287 + f64x8::splat(0.0366775) * t291 + f64x8::splat(0.073355) * t293 + f64x8::splat(0.137975) * t296;
            let t333 = t148 * t331 * t155;
            let t336 = t147 * t147;
            let t337 = f64x8::splat(1.0) / t336;
            let t338 = t337 * t320;
            let t339 = t53 * t53;
            let t340 = f64x8::splat(1.0) / t339;
            let t341 = t338 * t340;
            let t344 = -f64x8::splat(0.0014764627977777779) * t4 * t259 * t30 - f64x8::splat(0.035616666666666665) * t263 * t264 * t138 - f64x8::splat(2.0) * t270 * t272 + f64x8::splat(1.0) * t118 * t299 + f64x8::splat(16.081979498692537) * t304 * t307 + f64x8::splat(0.00024415263074675396) * t141 * t121 * t66 * t54 + f64x8::splat(0.01084358130030174) * t314 * t112 * t156 + f64x8::splat(1.1696447245269292) * t146 * t322 - f64x8::splat(0.5848223622634646) * t146 * t333 - f64x8::splat(17.315859105681465) * t146 * t341;
            let t345 = v_rho * t344;
            let t349 = t83 * t78;
            let t350 = t163 * t163;
            let t351 = f64x8::splat(1.0) / t350;
            let t352 = t180 * t180;
            let t353 = t351 * t352;
            let t356 = t195 * t195;
            let t357 = t356 * t94;
            let t360 = t163 * t76;
            let t361 = f64x8::splat(1.0) / t360;
            let t362 = t361 * t352;
            let t367 = t64 * t64;
            let t369 = f64x8::splat(1.0) / t7 / t367;
            let t371 = t63 * v_sigma * t369;
            let t377 = f64x8::splat(1.0) / t173 / t58;
            let t378 = t5 * t377;
            let t379 = t159 * t159;
            let t380 = t378 * t379;
            let t381 = t70 * t380;
            let t384 = t175 * t344;
            let t385 = t70 * t384;
            let t388 = f64x8::splat(0.005406067689814815) * t371 * t73 + f64x8::splat(0.003243640613888889) * t170 * t177 + f64x8::splat(0.0013901316916666666) * t68 * t381 - f64x8::splat(0.0006950658458333333) * t68 * t385;
            let t389 = t388 * t94;
            let t392 = t103 * t98;
            let t393 = t94 * t94;
            let t394 = t356 * t393;
            let t397 = t100 * t95;
            let t402 = t106 * t101;
            let t413 = -t388;
            let t414 = t164 * t413;
            let t417 = -f64x8::splat(4.0) * t182 * t362 + f64x8::splat(2.0) * t182 * t414 - f64x8::splat(6.0) * t186 * t362 - f64x8::splat(8.0) * t189 * t362 - f64x8::splat(10.0) * t192 * t362 - f64x8::splat(3.0) * t202 * t389 - f64x8::splat(4.0) * t205 * t357 - f64x8::splat(4.0) * t205 * t389 - f64x8::splat(5.0) * t208 * t357 - f64x8::splat(5.0) * t208 * t389 + f64x8::splat(6.0) * t349 * t353 + f64x8::splat(12.0) * t392 * t394 + f64x8::splat(6.0) * t397 * t394 + f64x8::splat(20.0) * t402 * t394;
            let t418 = t93 * t356;
            let t424 = t80 * t351;
            let t427 = t97 * t356;
            let t433 = t60 * t361;
            let t438 = t86 * t81;
            let t441 = t89 * t84;
            let t450 = -t93 * t388 * t94 + t165 * t413 + f64x8::splat(3.0) * t186 * t414 + f64x8::splat(4.0) * t189 * t414 + f64x8::splat(5.0) * t192 * t414 - f64x8::splat(2.0) * t198 * t357 - f64x8::splat(2.0) * t198 * t389 - f64x8::splat(3.0) * t202 * t357 + f64x8::splat(2.0) * t424 * t352 - f64x8::splat(2.0) * t433 * t352 + f64x8::splat(12.0) * t438 * t353 + f64x8::splat(20.0) * t441 * t353 + f64x8::splat(2.0) * t427 * t393 - t418 * t94;
            let t451 = t417 + t450;
            let tv2rho20 = f64x8::splat(2.0) * t159 * t109 + t345 * t109 + f64x8::splat(2.0) * t160 * t211 + t162 * t451 + f64x8::splat(2.0) * t58 * t211;
            acc_v2rho2 = tv2rho20;
            let t455 = t433 * t241;
            let t456 = t72 * t180;
            let t457 = t70 * t456;
            let t460 = t165 * t241;
            let t463 = t63 * t168;
            let t464 = t198 * t463;
            let t467 = t202 * t463;
            let t470 = t205 * t463;
            let t473 = t168 * t18;
            let t474 = t473 * t216;
            let t482 = t180 * t39 * t62;
            let t483 = t424 * t482;
            let t488 = t235 * t18;
            let t489 = t234 * t488;
            let t490 = t174 * t94;
            let t491 = t490 * t159;
            let t492 = t215 * t491;
            let t496 = t71 * t195 * t94;
            let t497 = t215 * t496;
            let t500 = t97 * t195;
            let t501 = t393 * t39;
            let t502 = t501 * t62;
            let t503 = t500 * t502;
            let t506 = t208 * t463;
            let t509 = t62 * t168;
            let t516 = t361 * t39 * t62;
            let t517 = t182 * t516;
            let t518 = t214 * t69;
            let t519 = t518 * t456;
            let t522 = t518 * t176;
            let t525 = t186 * t516;
            let t528 = f64x8::splat(0.0013901316916666666) * t455 * t457 + f64x8::splat(0.0006950658458333333) * t460 * t177 + f64x8::splat(0.003243640613888889) * t464 * t238 + f64x8::splat(0.0048654609208333335) * t467 * t238 + f64x8::splat(0.006487281227777778) * t470 * t238 + f64x8::splat(0.006487281227777778) * t228 * t474 + f64x8::splat(0.008109101534722222) * t231 * t474 + f64x8::splat(0.003243640613888889) * t222 * t474 - f64x8::splat(0.0013901316916666666) * t483 * t217 + f64x8::splat(0.0048654609208333335) * t225 * t474 + f64x8::splat(0.0006950658458333333) * t489 * t492 - f64x8::splat(0.0006950658458333333) * t489 * t497 + f64x8::splat(0.0013901316916666666) * t503 * t217 + f64x8::splat(0.008109101534722222) * t506 * t238 + f64x8::splat(0.0016218203069444444) * t234 * t509 * t238 + f64x8::splat(0.0016218203069444444) * t213 * t474 + f64x8::splat(0.0027802633833333332) * t517 * t519 + f64x8::splat(0.0013901316916666666) * t222 * t522 + f64x8::splat(0.004170395075) * t525 * t519;
            let t529 = t70 * t5;
            let t530 = t529 * t496;
            let t533 = t529 * t491;
            let t540 = t397 * t241;
            let t541 = t71 * t393;
            let t543 = t529 * t541 * t195;
            let t550 = t189 * t516;
            let t555 = t192 * t516;
            let t561 = t351 * t39 * t62;
            let t562 = t349 * t561;
            let t565 = t438 * t561;
            let t568 = t441 * t561;
            let t573 = t392 * t241;
            let t580 = t402 * t241;
            let t583 = -f64x8::splat(0.0034753292291666666) * t251 * t530 + f64x8::splat(0.0013901316916666666) * t242 * t533 - f64x8::splat(0.0013901316916666666) * t242 * t530 + f64x8::splat(0.0020851975375) * t245 * t533 + f64x8::splat(0.004170395075) * t540 * t543 - f64x8::splat(0.0020851975375) * t245 * t530 + f64x8::splat(0.0020851975375) * t225 * t522 + f64x8::splat(0.0055605267666666664) * t550 * t519 + f64x8::splat(0.0027802633833333332) * t228 * t522 + f64x8::splat(0.006950658458333333) * t555 * t519 + f64x8::splat(0.0034753292291666666) * t231 * t522 - f64x8::splat(0.004170395075) * t562 * t519 - f64x8::splat(0.00834079015) * t565 * t519 - f64x8::splat(0.013901316916666667) * t568 * t519 + f64x8::splat(0.0027802633833333332) * t248 * t533 + f64x8::splat(0.00834079015) * t573 * t543 - f64x8::splat(0.0027802633833333332) * t248 * t530 + f64x8::splat(0.0034753292291666666) * t251 * t533 + f64x8::splat(0.013901316916666667) * t580 * t543;
            let t584 = t528 + t583;
            let tv2rhosigma0 = t160 * t254 + t162 * t584 + t58 * t254;
            acc_v2rhosigma = tv2rhosigma0;
            let t586 = t39 * t39;
            let t587 = t62 * t62;
            let t588 = t586 * t587;
            let t589 = t433 * t588;
            let t591 = f64x8::splat(1.0) / t21 / t367;
            let t592 = t591 * t1;
            let t593 = f64x8::splat(1.0) / t19;
            let t594 = t593 * t6;
            let t595 = t594 * t174;
            let t596 = t592 * t595;
            let t599 = t424 * t588;
            let t603 = t361 * t586 * t587;
            let t604 = t182 * t603;
            let t608 = t351 * t586 * t587;
            let t609 = t349 * t608;
            let t612 = t186 * t603;
            let t615 = t438 * t608;
            let t618 = t189 * t603;
            let t621 = t441 * t608;
            let t624 = t192 * t603;
            let t627 = t93 * t586;
            let t628 = t587 * t591;
            let t630 = t1 * t593;
            let t631 = t6 * t174;
            let t633 = t630 * t631 * t94;
            let t636 = t97 * t586;
            let t639 = t630 * t631 * t393;
            let t642 = t588 * t591;
            let t643 = t198 * t642;
            let t646 = t397 * t642;
            let t649 = t202 * t642;
            let t652 = t392 * t642;
            let t655 = t205 * t642;
            let t658 = t402 * t642;
            let t661 = t208 * t642;
            let t664 = -f64x8::splat(2.8986991802640425e-06) * t589 * t596 + f64x8::splat(2.8986991802640425e-06) * t599 * t596 - f64x8::splat(5.797398360528085e-06) * t604 * t596 + f64x8::splat(8.696097540792127e-06) * t609 * t596 - f64x8::splat(8.696097540792127e-06) * t612 * t596 + f64x8::splat(1.7392195081584254e-05) * t615 * t596 - f64x8::splat(1.159479672105617e-05) * t618 * t596 + f64x8::splat(2.8986991802640426e-05) * t621 * t596 - f64x8::splat(1.4493495901320213e-05) * t624 * t596 - f64x8::splat(1.4493495901320212e-06) * t627 * t628 * t633 + f64x8::splat(2.8986991802640425e-06) * t636 * t628 * t639 - f64x8::splat(2.8986991802640425e-06) * t643 * t633 + f64x8::splat(8.696097540792127e-06) * t646 * t639 - f64x8::splat(4.3480487703960635e-06) * t649 * t633 + f64x8::splat(1.7392195081584254e-05) * t652 * t639 - f64x8::splat(5.797398360528085e-06) * t655 * t633 + f64x8::splat(2.8986991802640426e-05) * t658 * t639 - f64x8::splat(7.246747950660106e-06) * t661 * t633;
            let tv2sigma20 = t162 * t664;
            acc_v2sigma2 = tv2sigma20;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rho2.into(); v2rho2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rhosigma.into(); v2rhosigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2sigma2.into(); v2sigma2[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
