//! LDA_C_PMGB06 fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_pmgb06.c`
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
pub fn lda_c_pmgb06_fxc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_hyb_omega_0 = f64x8::splat(param_hyb_omega_0);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_v2rho2 = V_ZERO;
        {
            let t1 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t2 = (simd::cbrt(zeta_threshold));
            let t3 = t2 * t2;
            let t4 = ((t1).select(t3, f64x8::splat(1.0)));
            let t5 = t4 * t4;
            let t6 = t5 * t4;
            let t7 = (simd::ln(f64x8::splat(2.0)));
            let t8 = t7 - f64x8::splat(1.0);
            let t10 = f64x8::splat(2.0) * t6 * t8;
            let t11 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t12 = f64x8::splat(1.0) / t11;
            let t13 = f64x8::splat(M_CBRT3);
            let t14 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t15 = (simd::cbrt(t14));
            let t16 = t13 * t15;
            let t17 = f64x8::splat(M_CBRT4);
            let t18 = t17 * t17;
            let t19 = (simd::cbrt(v_rho));
            let t20 = f64x8::splat(1.0) / t19;
            let t21 = t18 * t20;
            let t22 = t16 * t21;
            let t23 = ((t22).sqrt());
            let t25 = f64x8::splat(1.0) / t4;
            let t27 = f64x8::splat(2.923025) * param_hyb_omega_0 * t23 * t25;
            let t29 = (simd::cbrt(f64x8::splat(9.0)));
            let t30 = t29 * t29;
            let t38 = param_hyb_omega_0 * param_hyb_omega_0;
            let t40 = (f64x8::splat(3.44851) - f64x8::splat(M_PI) * t17 * t30 * t15 / t8 / f64x8::splat(12.0)) * t38 * t13;
            let t41 = t15 * t18;
            let t42 = f64x8::splat(1.0) / t5;
            let t47 = t38 * param_hyb_omega_0;
            let t48 = t23 * t22;
            let t50 = f64x8::splat(1.0) / t6;
            let t53 = f64x8::splat(1.0) + t27 + t40 * t41 * t20 * t42 / f64x8::splat(4.0) + f64x8::splat(0.48968) * t47 * t48 * t50;
            let t55 = t38 * t13 * t15;
            let t59 = f64x8::splat(1.0) + t27 + f64x8::splat(0.8621275) * t55 * t21 * t42;
            let t60 = f64x8::splat(1.0) / t59;
            let t62 = (simd::ln(t53 * t60));
            let t65 = f64x8::splat(1.0) / v_rho;
            let t74 = (f64x8::splat(2.0) / f64x8::splat(45.0) * t17 * t30 * t15 * (t11 + f64x8::splat(6.0) * t7 - f64x8::splat(3.0)) * t14 - f64x8::splat(0.7524)) * t13;
            let t78 = t13 * t13;
            let t79 = t15 * t15;
            let t80 = t78 * t79;
            let t81 = t19 * t19;
            let t82 = f64x8::splat(1.0) / t81;
            let t83 = t17 * t82;
            let t84 = t80 * t83;
            let t87 = t15 * t14;
            let t88 = t13 * t87;
            let t90 = f64x8::splat(1.0) / t19 / v_rho;
            let t91 = t18 * t90;
            let t94 = f64x8::splat(1.0) - t74 * t41 * t20 / f64x8::splat(4.0) + f64x8::splat(0.0204825) * t84 - f64x8::splat(0.0030486129349252553) * t65 + f64x8::splat(0.0003485625) * t88 * t91;
            let t97 = (simd::exp(-f64x8::splat(0.1881) * t22));
            let t98 = f64x8::splat(M_SQRT2);
            let t99 = t97 * t98;
            let t103 = t78 * t79 * t12;
            let t104 = t103 * t17;
            let t106 = f64x8::splat(1.0) / t81 / v_rho;
            let t107 = zeta_threshold * zeta_threshold;
            let t108 = ((t1).select(t107, f64x8::splat(1.0)));
            let t109 = t108 * t30;
            let t110 = f64x8::splat(1.0) / t87;
            let t111 = t109 * t110;
            let t113 = f64x8::splat(M_CBRT2);
            let t115 = t16 * t21 * t113;
            let t117 = f64x8::splat(1.0) - f64x8::splat(0.0056675) * t115;
            let t119 = t113 * t113;
            let t123 = f64x8::splat(1.0) + f64x8::splat(0.107975) * t115 + f64x8::splat(0.01) * t80 * t83 * t119;
            let t124 = f64x8::splat(1.0) / t123;
            let t125 = t117 * t124;
            let t128 = t111 * t13 * t81 * t125 / f64x8::splat(15.0);
            let t131 = -f64x8::splat(1.2375) * t22 + t84 / f64x8::splat(4.0);
            let t133 = (simd::exp(-f64x8::splat(0.0775) * t22));
            let t134 = t131 * t133;
            let t135 = f64x8::splat(M_PI) * v_rho;
            let t138 = t128 + f64x8::splat(4.0) / f64x8::splat(3.0) * t134 * t135;
            let t145 = t94 * t97;
            let t147 = t145 / f64x8::splat(2.0) - f64x8::splat(1.0) / f64x8::splat(2.0);
            let t150 = t17 * t106;
            let t153 = -f64x8::splat(0.097) * t22 + f64x8::splat(0.169) * t84;
            let t155 = (simd::exp(-f64x8::splat(0.13675) * t22));
            let t157 = t153 * t155 * t13;
            let t159 = f64x8::splat(1.0) / t79 * t18;
            let t160 = t159 * t81;
            let t164 = ((t1).select(t3 * t107, f64x8::splat(1.0)));
            let t165 = t164 * t30;
            let t166 = t110 * t13;
            let t170 = t128 + t157 * t160 / f64x8::splat(3.0) - t165 * t166 * t81 / f64x8::splat(15.0);
            let t175 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t22;
            let t178 = ((t22) * (t22).sqrt());
            let t181 = f64x8::splat(3.79785) * t23 + f64x8::splat(0.8969) * t22 + f64x8::splat(0.204775) * t178 + f64x8::splat(0.123235) * t84;
            let t184 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t181;
            let t185 = (simd::ln(t184));
            let t189 = ((t1).select(t2 * zeta_threshold, f64x8::splat(1.0)));
            let t195 = (f64x8::splat(2.0) * t189 - f64x8::splat(2.0)) / (f64x8::splat(2.0) * t113 - f64x8::splat(2.0));
            let t197 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t22;
            let t202 = f64x8::splat(5.1785) * t23 + f64x8::splat(0.905775) * t22 + f64x8::splat(0.1100325) * t178 + f64x8::splat(0.1241775) * t84;
            let t205 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t202;
            let t206 = (simd::ln(t205));
            let t210 = -f64x8::splat(0.0621814) * t175 * t185 + f64x8::splat(0.0197516734986138) * t195 * t197 * t206;
            let t215 = t38 * t38;
            let t217 = t103 * t150;
            let t218 = t215 * param_hyb_omega_0;
            let t219 = t98 * t218;
            let t220 = t145 * t219;
            let t226 = v_rho * v_rho;
            let t227 = f64x8::splat(1.0) / t226;
            let t231 = t215 * t38;
            let t234 = f64x8::splat(1.0) / t81 / t226;
            let t236 = t215 * t215;
            let t240 = t10 * t12 * t62 + (-f64x8::splat(0.031505407223141116) * t65 * t94 * t99 - f64x8::splat(0.005388405304614574) * t104 * t106 * t138 * t98) * t47 + (-f64x8::splat(0.0837628205355044) * t65 * t147 - f64x8::splat(0.011938374665504766) * t103 * t150 * t170 + f64x8::splat(0.42708890021612717) * t88 * t91 * t210) * t215 - f64x8::splat(0.01197423401025461) * t217 * t220 + (-f64x8::splat(0.031835665774679375) * t103 * t150 * t147 + f64x8::splat(0.05332506774217938) * t227 * t210) * t231 + f64x8::splat(0.020267214298646783) * t104 * t234 * t210 * t236;
            let t244 = f64x8::splat(1.0) + f64x8::splat(0.15403623315025) * t80 * t83 * t38;
            let t245 = t244 * t244;
            let t246 = t245 * t245;
            let t247 = f64x8::splat(1.0) / t246;
            let tzk0 = t240 * t247;
            acc_zk = tzk0;
            let t248 = t10 * t12;
            let t249 = f64x8::splat(1.0) / t23;
            let t251 = param_hyb_omega_0 * t249 * t25;
            let t252 = t16 * t91;
            let t254 = f64x8::splat(0.48717083333333333) * t251 * t252;
            let t260 = t47 * t23 * t50;
            let t263 = -t254 - t40 * t41 * t90 * t42 / f64x8::splat(12.0) - f64x8::splat(0.24484) * t260 * t252;
            let t265 = t59 * t59;
            let t266 = f64x8::splat(1.0) / t265;
            let t267 = t53 * t266;
            let t271 = -t254 - f64x8::splat(0.28737583333333333) * t55 * t91 * t42;
            let t273 = t263 * t60 - t267 * t271;
            let t274 = f64x8::splat(1.0) / t53;
            let t275 = t273 * t274;
            let t281 = t41 * t90;
            let t284 = t80 * t150;
            let t288 = f64x8::splat(1.0) / t19 / t226;
            let t289 = t18 * t288;
            let t292 = t74 * t281 / f64x8::splat(12.0) - f64x8::splat(0.013655) * t284 + f64x8::splat(0.0030486129349252553) * t227 - f64x8::splat(0.00046475) * t88 * t289;
            let t298 = t41 * t99;
            let t308 = f64x8::splat(2.0) / f64x8::splat(45.0) * t111 * t13 * t20 * t125;
            let t309 = t109 * t78;
            let t311 = t113 * t124;
            let t314 = f64x8::splat(0.0003956661414271145) * t309 * t82 * t18 * t311;
            let t315 = t109 * t166;
            let t316 = t81 * t117;
            let t317 = t123 * t123;
            let t318 = f64x8::splat(1.0) / t317;
            let t325 = -f64x8::splat(0.035991666666666665) * t16 * t91 * t113 - f64x8::splat(0.006666666666666667) * t80 * t150 * t119;
            let t326 = t318 * t325;
            let t329 = t315 * t316 * t326 / f64x8::splat(15.0);
            let t332 = f64x8::splat(0.4125) * t252 - t284 / f64x8::splat(6.0);
            let t333 = t332 * t133;
            let t337 = t131 * t13 * t15;
            let t338 = t21 * t133;
            let t343 = t308 + t314 - t329 + f64x8::splat(4.0) / f64x8::splat(3.0) * t333 * t135 + f64x8::splat(0.10821041362364843) * t337 * t338 + f64x8::splat(4.0) / f64x8::splat(3.0) * t134 * f64x8::splat(M_PI);
            let t352 = t292 * t97;
            let t355 = t94 * t13 * t15;
            let t356 = t91 * t97;
            let t359 = t352 / f64x8::splat(2.0) + f64x8::splat(0.03135) * t355 * t356;
            let t362 = t17 * t234;
            let t368 = f64x8::splat(0.03233333333333333) * t252 - f64x8::splat(0.11266666666666666) * t284;
            let t370 = t368 * t155 * t13;
            let t374 = f64x8::splat(1.0) / t15;
            let t375 = t153 * t78 * t374;
            let t376 = t83 * t155;
            let t379 = t159 * t20;
            let t385 = t308 + t314 - t329 + t370 * t160 / f64x8::splat(3.0) + f64x8::splat(0.06077777777777778) * t375 * t376 + f64x8::splat(2.0) / f64x8::splat(9.0) * t157 * t379 - f64x8::splat(2.0) / f64x8::splat(45.0) * t165 * t166 * t20;
            let t395 = t181 * t181;
            let t396 = f64x8::splat(1.0) / t395;
            let t397 = t175 * t396;
            let t398 = t249 * t13;
            let t399 = t398 * t281;
            let t402 = ((t22).sqrt());
            let t403 = t402 * t13;
            let t404 = t403 * t281;
            let t407 = -f64x8::splat(0.632975) * t399 - f64x8::splat(0.29896666666666666) * t252 - f64x8::splat(0.1023875) * t404 - f64x8::splat(0.08215666666666667) * t284;
            let t408 = f64x8::splat(1.0) / t184;
            let t409 = t407 * t408;
            let t412 = t195 * t13;
            let t417 = t195 * t197;
            let t418 = t202 * t202;
            let t419 = f64x8::splat(1.0) / t418;
            let t424 = -f64x8::splat(0.8630833333333333) * t399 - f64x8::splat(0.301925) * t252 - f64x8::splat(0.05501625) * t404 - f64x8::splat(0.082785) * t284;
            let t426 = f64x8::splat(1.0) / t205;
            let t427 = t419 * t424 * t426;
            let t430 = f64x8::splat(0.0011073470983333333) * t16 * t91 * t185 + f64x8::splat(1.0) * t397 * t409 - f64x8::splat(0.00018311447306006544) * t412 * t41 * t90 * t206 - f64x8::splat(0.5848223622634646) * t417 * t427;
            let t436 = t103 * t362;
            let t439 = t352 * t219;
            let t442 = t226 * v_rho;
            let t443 = f64x8::splat(1.0) / t442;
            let t444 = t443 * t94;
            let t445 = t99 * t218;
            let t461 = f64x8::splat(1.0) / t81 / t442;
            let t470 = t248 * t275 * t59 + (f64x8::splat(0.031505407223141116) * t227 * t94 * t99 - f64x8::splat(0.031505407223141116) * t65 * t292 * t99 - f64x8::splat(0.001975389032890948) * t288 * t94 * t13 * t298 + f64x8::splat(0.008980675507690957) * t104 * t234 * t138 * t98 - f64x8::splat(0.005388405304614574) * t104 * t106 * t343 * t98) * t47 + (f64x8::splat(0.0837628205355044) * t227 * t147 - f64x8::splat(0.0837628205355044) * t65 * t359 + f64x8::splat(0.019897291109174608) * t103 * t362 * t170 - f64x8::splat(0.011938374665504766) * t103 * t150 * t385 - f64x8::splat(0.5694518669548363) * t88 * t289 * t210 + f64x8::splat(0.42708890021612717) * t88 * t91 * t430) * t215 + f64x8::splat(0.019957056683757683) * t436 * t220 - f64x8::splat(0.01197423401025461) * t217 * t439 - f64x8::splat(0.0002905674151788692) * t444 * t445 + (f64x8::splat(0.053059442957798957) * t103 * t362 * t147 - f64x8::splat(0.031835665774679375) * t103 * t150 * t359 - f64x8::splat(0.10665013548435875) * t443 * t210 + f64x8::splat(0.05332506774217938) * t227 * t430) * t231 - f64x8::splat(0.054045904796391424) * t104 * t461 * t210 * t236 + f64x8::splat(0.020267214298646783) * t104 * t234 * t430 * t236;
            let t475 = f64x8::splat(1.0) / t246 / t244;
            let t478 = t80 * t17 * t38;
            let tvrho0 = tzk0 + v_rho * t470 * t247 + f64x8::splat(0.41076328840066667) * t82 * t240 * t475 * t478;
            acc_vrho = tvrho0;
            let t484 = t240 * t475 * t78;
            let t485 = t79 * t17;
            let t487 = t485 * t106 * t38;
            let t490 = f64x8::splat(1.0) / t48;
            let t492 = param_hyb_omega_0 * t490 * t25;
            let t493 = t80 * t362;
            let t495 = f64x8::splat(0.3247805555555556) * t492 * t493;
            let t496 = t16 * t289;
            let t498 = f64x8::splat(0.6495611111111111) * t251 * t496;
            let t504 = t47 * t249 * t50;
            let t509 = -t495 + t498 + t40 * t41 * t288 * t42 / f64x8::splat(9.0) + f64x8::splat(0.16322666666666666) * t504 * t493 + f64x8::splat(0.3264533333333333) * t260 * t496;
            let t511 = t263 * t266;
            let t515 = f64x8::splat(1.0) / t265 / t59;
            let t516 = t53 * t515;
            let t517 = t271 * t271;
            let t523 = -t495 + t498 + f64x8::splat(0.3831677777777778) * t55 * t289 * t42;
            let t525 = -t267 * t523 - f64x8::splat(2.0) * t511 * t271 + t509 * t60 + f64x8::splat(2.0) * t516 * t517;
            let t526 = t525 * t274;
            let t529 = t53 * t53;
            let t530 = f64x8::splat(1.0) / t529;
            let t531 = t273 * t530;
            let t532 = t59 * t263;
            let t543 = f64x8::splat(1.0) / t19 / t442;
            let t548 = t41 * t288;
            let t553 = t18 * t543;
            let t556 = -t74 * t548 / f64x8::splat(9.0) + f64x8::splat(0.022758333333333332) * t493 - f64x8::splat(0.006097225869850511) * t443 + f64x8::splat(0.0010844166666666667) * t88 * t553;
            let t566 = t485 * t99;
            let t580 = f64x8::splat(2.0) / f64x8::splat(135.0) * t111 * t13 * t90 * t125;
            let t581 = t20 * t117;
            let t584 = f64x8::splat(4.0) / f64x8::splat(45.0) * t315 * t581 * t326;
            let t586 = t109 * t78 * t82;
            let t587 = t18 * t113;
            let t588 = t587 * t326;
            let t590 = f64x8::splat(0.000791332282854229) * t586 * t588;
            let t592 = f64x8::splat(1.0) / t317 / t123;
            let t593 = t325 * t325;
            let t594 = t592 * t593;
            let t597 = f64x8::splat(2.0) / f64x8::splat(15.0) * t315 * t316 * t594;
            let t604 = f64x8::splat(0.047988888888888886) * t16 * t289 * t113 + f64x8::splat(0.011111111111111112) * t80 * t362 * t119;
            let t605 = t318 * t604;
            let t608 = t315 * t316 * t605 / f64x8::splat(15.0);
            let t611 = -f64x8::splat(0.55) * t496 + f64x8::splat(5.0) / f64x8::splat(18.0) * t493;
            let t612 = t611 * t133;
            let t616 = t332 * t13 * t15;
            let t621 = t91 * t133;
            let t625 = t131 * t78 * t79;
            let t626 = t150 * t133;
            let t629 = -t580 - t584 - t590 + t597 - t608 + f64x8::splat(4.0) / f64x8::splat(3.0) * t612 * t135 + f64x8::splat(0.21642082724729686) * t616 * t338 + f64x8::splat(8.0) / f64x8::splat(3.0) * t333 * f64x8::splat(M_PI) + f64x8::splat(0.07214027574909895) * t337 * t621 + f64x8::splat(0.011181742741110338) * t625 * t626;
            let t640 = t556 * t97;
            let t643 = t292 * t13 * t15;
            let t646 = t289 * t97;
            let t650 = t94 * t78 * t79;
            let t651 = t362 * t97;
            let t654 = t640 / f64x8::splat(2.0) + f64x8::splat(0.0627) * t643 * t356 - f64x8::splat(0.0418) * t355 * t646 + f64x8::splat(0.00786258) * t650 * t651;
            let t657 = t17 * t461;
            let t666 = -f64x8::splat(0.043111111111111114) * t496 + f64x8::splat(0.18777777777777777) * t493;
            let t668 = t666 * t155 * t13;
            let t672 = t368 * t78 * t374;
            let t680 = t159 * t90;
            let t686 = -t580 - t584 - t590 + t597 - t608 + t668 * t160 / f64x8::splat(3.0) + f64x8::splat(0.12155555555555556) * t672 * t376 + f64x8::splat(4.0) / f64x8::splat(9.0) * t370 * t379 + f64x8::splat(0.033245444444444446) * t153 * t227 * t155 - f64x8::splat(2.0) / f64x8::splat(27.0) * t157 * t680 + f64x8::splat(2.0) / f64x8::splat(135.0) * t165 * t166 * t90;
            let t699 = t16 * t18;
            let t700 = t90 * t396;
            let t704 = t395 * t181;
            let t705 = f64x8::splat(1.0) / t704;
            let t706 = t175 * t705;
            let t707 = t407 * t407;
            let t708 = t707 * t408;
            let t711 = t490 * t78;
            let t712 = t485 * t234;
            let t713 = t711 * t712;
            let t715 = t398 * t548;
            let t718 = f64x8::splat(1.0)/((t22).sqrt());
            let t719 = t718 * t78;
            let t720 = t719 * t712;
            let t722 = t403 * t548;
            let t725 = -f64x8::splat(0.4219833333333333) * t713 + f64x8::splat(0.8439666666666666) * t715 + f64x8::splat(0.3986222222222222) * t496 + f64x8::splat(0.06825833333333334) * t720 + f64x8::splat(0.13651666666666668) * t722 + f64x8::splat(0.1369277777777778) * t493;
            let t726 = t725 * t408;
            let t729 = t395 * t395;
            let t730 = f64x8::splat(1.0) / t729;
            let t731 = t175 * t730;
            let t732 = t184 * t184;
            let t733 = f64x8::splat(1.0) / t732;
            let t734 = t707 * t733;
            let t741 = t195 * t16;
            let t745 = t418 * t202;
            let t746 = f64x8::splat(1.0) / t745;
            let t747 = t424 * t424;
            let t749 = t746 * t747 * t426;
            let t758 = -f64x8::splat(0.5753888888888888) * t713 + f64x8::splat(1.1507777777777777) * t715 + f64x8::splat(0.4025666666666667) * t496 + f64x8::splat(0.0366775) * t720 + f64x8::splat(0.073355) * t722 + f64x8::splat(0.137975) * t493;
            let t760 = t419 * t758 * t426;
            let t763 = t418 * t418;
            let t764 = f64x8::splat(1.0) / t763;
            let t765 = t764 * t747;
            let t766 = t205 * t205;
            let t767 = f64x8::splat(1.0) / t766;
            let t768 = t765 * t767;
            let t771 = -f64x8::splat(0.0014764627977777779) * t16 * t289 * t185 - f64x8::splat(0.035616666666666665) * t699 * t700 * t409 - f64x8::splat(2.0) * t706 * t708 + f64x8::splat(1.0) * t397 * t726 + f64x8::splat(16.081979498692537) * t731 * t734 + f64x8::splat(0.00024415263074675396) * t412 * t41 * t288 * t206 + f64x8::splat(0.01084358130030174) * t741 * t91 * t427 + f64x8::splat(1.1696447245269292) * t417 * t749 - f64x8::splat(0.5848223622634646) * t417 * t760 - f64x8::splat(17.315859105681465) * t417 * t768;
            let t777 = t103 * t657;
            let t782 = t226 * t226;
            let t783 = f64x8::splat(1.0) / t782;
            let t784 = t783 * t94;
            let t787 = t640 * t219;
            let t790 = t443 * t292;
            let t794 = f64x8::splat(1.0) / t19 / t782;
            let t795 = t794 * t94;
            let t798 = t18 * t97 * t219;
            let t819 = f64x8::splat(1.0) / t81 / t782;
            let t832 = t248 * t526 * t59 - t248 * t531 * t532 + t248 * t275 * t271 + (-f64x8::splat(0.06301081444628223) * t444 * t99 + f64x8::splat(0.06301081444628223) * t227 * t292 * t99 + f64x8::splat(0.006584630109636494) * t543 * t94 * t13 * t298 - f64x8::splat(0.031505407223141116) * t65 * t556 * t99 - f64x8::splat(0.003950778065781896) * t288 * t292 * t13 * t298 - f64x8::splat(0.0004954275694490498) * t461 * t94 * t78 * t566 - f64x8::splat(0.02394846802050922) * t104 * t461 * t138 * t98 + f64x8::splat(0.017961351015381915) * t104 * t234 * t343 * t98 - f64x8::splat(0.005388405304614574) * t104 * t106 * t629 * t98) * t47 + (-f64x8::splat(0.1675256410710088) * t443 * t147 + f64x8::splat(0.1675256410710088) * t227 * t359 - f64x8::splat(0.0837628205355044) * t65 * t654 - f64x8::splat(0.053059442957798957) * t103 * t657 * t170 + f64x8::splat(0.039794582218349216) * t103 * t362 * t385 - f64x8::splat(0.011938374665504766) * t103 * t150 * t686 + f64x8::splat(1.328721022894618) * t88 * t553 * t210 - f64x8::splat(1.1389037339096726) * t88 * t289 * t430 + f64x8::splat(0.42708890021612717) * t88 * t91 * t771) * t215 - f64x8::splat(0.05321881782335382) * t777 * t220 + f64x8::splat(0.039914113367515366) * t436 * t439 + f64x8::splat(0.001355981270834723) * t784 * t445 - f64x8::splat(0.01197423401025461) * t217 * t787 - f64x8::splat(0.0005811348303577384) * t790 * t445 - f64x8::splat(1.82185769317151e-05) * t795 * t16 * t798 + (-f64x8::splat(0.14149184788746388) * t103 * t657 * t147 + f64x8::splat(0.10611888591559791) * t103 * t362 * t359 - f64x8::splat(0.031835665774679375) * t103 * t150 * t654 + f64x8::splat(0.31995040645307626) * t783 * t210 - f64x8::splat(0.2133002709687175) * t443 * t430 + f64x8::splat(0.05332506774217938) * t227 * t771) * t231 + f64x8::splat(0.19816831758676853) * t104 * t819 * t210 * t236 - f64x8::splat(0.10809180959278285) * t104 * t461 * t430 * t236 + f64x8::splat(0.020267214298646783) * t104 * t234 * t771 * t236;
            let t841 = f64x8::splat(1.0) / t246 / t245;
            let t844 = t88 * t18 * t215;
            let tv2rho20 = f64x8::splat(2.0) * t470 * t247 + f64x8::splat(0.13692109613355555) * t484 * t487 + v_rho * t832 * t247 + f64x8::splat(0.8215265768013333) * t82 * t470 * t475 * t478 + f64x8::splat(0.6327242966164848) * t288 * t240 * t841 * t844;
            acc_v2rho2 = tv2rho20;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(v2rho2, ip, m, acc_v2rho2);
        ip += 8;
    }
}
