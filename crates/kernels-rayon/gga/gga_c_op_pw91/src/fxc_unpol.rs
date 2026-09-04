//! GGA_C_OP_PW91 fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_op_pw91.c`
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
pub fn gga_c_op_pw91_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
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
        {
            let t1 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t4 = (t1) | ((v_rho / f64x8::splat(2.0)).simd_le(dens_threshold));
            let t5 = zeta_threshold - f64x8::splat(1.0);
            let t6 = -t5;
            let t7 = ((t1).select(t5, (t1).select(t6, f64x8::splat(0.0))));
            let t8 = t7 * t7;
            let t9 = f64x8::splat(1.0) - t8;
            let t10 = t9 * v_rho;
            let t11 = f64x8::splat(1.0) + t7;
            let t14 = (t11 * v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t15 = f64x8::splat(M_CBRT3);
            let t16 = t15 * t15;
            let t18 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t20 = t16 / t18;
            let t21 = f64x8::splat(M_CBRT4);
            let t22 = t20 * t21;
            let t23 = f64x8::splat(M_CBRT2);
            let t24 = (t11).simd_le(zeta_threshold);
            let t25 = f64x8::splat(1.0) - t7;
            let t26 = (t25).simd_le(zeta_threshold);
            let t27 = ((t24).select(t5, (t26).select(t6, t7)));
            let t28 = f64x8::splat(1.0) + t27;
            let t29 = t28 * v_rho;
            let t30 = (simd::cbrt(t29));
            let t31 = f64x8::splat(1.0) / t30;
            let t32 = t23 * t31;
            let t33 = f64x8::splat(M_CBRT6);
            let t34 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t35 = (simd::cbrt(t34));
            let t36 = t35 * t35;
            let t37 = f64x8::splat(1.0) / t36;
            let t38 = t33 * t37;
            let t39 = t23 * t23;
            let t40 = v_sigma * t39;
            let t41 = v_rho * v_rho;
            let t42 = (simd::cbrt(v_rho));
            let t43 = t42 * t42;
            let t45 = f64x8::splat(1.0) / t43 / t41;
            let t46 = t40 * t45;
            let t47 = t38 * t46;
            let t49 = (simd::exp(-f64x8::splat(25.0) / f64x8::splat(6.0) * t47));
            let t52 = (f64x8::splat(0.2743) - f64x8::splat(0.1508) * t49) * t33;
            let t53 = t52 * t37;
            let t56 = t33 * t33;
            let t58 = f64x8::splat(1.0) / t35 / t34;
            let t59 = t56 * t58;
            let t60 = v_sigma * v_sigma;
            let t61 = t60 * t23;
            let t62 = t41 * t41;
            let t63 = t62 * v_rho;
            let t65 = f64x8::splat(1.0) / t42 / t63;
            let t68 = f64x8::splat(1.388888888888889e-05) * t59 * t61 * t65;
            let t69 = t53 * t46 / f64x8::splat(24.0) - t68;
            let t71 = t56 / t35;
            let t72 = ((v_sigma).sqrt());
            let t73 = t71 * t72;
            let t75 = f64x8::splat(1.0) / t42 / v_rho;
            let t81 = (simd::ln(f64x8::splat(0.6496333333333333) * t71 * t72 * t23 * t75 + ((((f64x8::splat(0.6496333333333333) * t71 * t72 * t23 * t75) * (f64x8::splat(0.6496333333333333) * t71 * t72 * t23 * t75)) + f64x8::splat(1.0)).sqrt())));
            let t82 = t23 * t75 * t81;
            let t85 = f64x8::splat(1.0) + f64x8::splat(0.016370833333333334) * t73 * t82 + t68;
            let t86 = f64x8::splat(1.0) / t85;
            let t88 = t69 * t86 + f64x8::splat(1.0);
            let t89 = f64x8::splat(1.0) / t88;
            let t93 = ((t14).select(f64x8::splat(0.0), t22 * t32 * t89 / f64x8::splat(9.0)));
            let t97 = (t25 * v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t98 = ((t26).select(t5, (t24).select(t6, -t7)));
            let t99 = f64x8::splat(1.0) + t98;
            let t100 = t99 * v_rho;
            let t101 = (simd::cbrt(t100));
            let t102 = f64x8::splat(1.0) / t101;
            let t103 = t23 * t102;
            let t107 = ((t97).select(f64x8::splat(0.0), t22 * t103 * t89 / f64x8::splat(9.0)));
            let t108 = t93 + t107;
            let t109 = (t108).simd_eq(f64x8::splat(0.0));
            let t110 = ((t109).select(f64x8::splat(f64::EPSILON), t108));
            let t113 = f64x8::splat(3.60663084) / t110 + f64x8::splat(0.5764);
            let t114 = t110 * t110;
            let t115 = t114 * t114;
            let t116 = f64x8::splat(1.0) / t115;
            let t118 = t114 * t110;
            let t119 = f64x8::splat(1.0) / t118;
            let t121 = f64x8::splat(1.0) / t114;
            let t123 = f64x8::splat(31.58152667175181) * t116 + f64x8::splat(15.032732091624375) * t119 + f64x8::splat(1.788764629788) * t121;
            let t124 = f64x8::splat(1.0) / t123;
            let tzk0 = ((t4).select(f64x8::splat(0.0), -f64x8::splat(0.25) * t10 * t113 * t124));
            acc_zk = tzk0;
            let t128 = t9 * t113;
            let t132 = f64x8::splat(1.0) / t30 / t29;
            let t138 = t88 * t88;
            let t139 = f64x8::splat(1.0) / t138;
            let t140 = t59 * t60;
            let t141 = t62 * t41;
            let t143 = f64x8::splat(1.0) / t42 / t141;
            let t144 = t23 * t143;
            let t145 = t144 * t49;
            let t148 = t41 * v_rho;
            let t150 = f64x8::splat(1.0) / t43 / t148;
            let t156 = f64x8::splat(7.407407407407407e-05) * t59 * t61 * t143;
            let t157 = -f64x8::splat(0.13962962962962963) * t140 * t145 - t53 * t40 * t150 / f64x8::splat(9.0) + t156;
            let t159 = t85 * t85;
            let t160 = f64x8::splat(1.0) / t159;
            let t161 = t69 * t160;
            let t165 = t23 / t42 / t41 * t81;
            let t168 = t38 * v_sigma;
            let t171 = f64x8::splat(2.532140806666667) * t47 + f64x8::splat(1.0);
            let t172 = ((t171).sqrt());
            let t173 = f64x8::splat(1.0) / t172;
            let t174 = t39 * t150 * t173;
            let t177 = -f64x8::splat(0.02182777777777778) * t73 * t165 - f64x8::splat(0.08508031222222222) * t168 * t174 - t156;
            let t179 = t157 * t86 - t161 * t177;
            let t180 = t139 * t179;
            let t185 = ((t14).select(f64x8::splat(0.0), -t22 * t23 * t132 * t89 * t28 / f64x8::splat(27.0) - t22 * t32 * t180 / f64x8::splat(9.0)));
            let t187 = f64x8::splat(1.0) / t101 / t100;
            let t197 = ((t97).select(f64x8::splat(0.0), -t22 * t23 * t187 * t89 * t99 / f64x8::splat(27.0) - t22 * t103 * t180 / f64x8::splat(9.0)));
            let t199 = ((t109).select(f64x8::splat(0.0), t185 + t197));
            let t204 = t123 * t123;
            let t205 = f64x8::splat(1.0) / t204;
            let t206 = t113 * t205;
            let t208 = f64x8::splat(1.0) / t115 / t110;
            let t209 = t208 * t199;
            let t211 = t116 * t199;
            let t215 = -f64x8::splat(126.32610668700724) * t209 - f64x8::splat(45.098196274873125) * t211 - f64x8::splat(3.577529259576) * t119 * t199;
            let t220 = ((t4).select(f64x8::splat(0.0), -f64x8::splat(0.25) * t128 * t124 + f64x8::splat(0.90165771) * t10 * t121 * t199 * t124 + f64x8::splat(0.25) * t10 * t206 * t215));
            let tvrho0 = v_rho * t220 + tzk0;
            acc_vrho = tvrho0;
            let t222 = t59 * t23;
            let t227 = t37 * t39;
            let t231 = v_sigma * t23;
            let t234 = f64x8::splat(2.777777777777778e-05) * t59 * t231 * t65;
            let t235 = f64x8::splat(0.05236111111111111) * t222 * t65 * t49 * v_sigma + t52 * t227 * t45 / f64x8::splat(24.0) - t234;
            let t238 = t71 / t72;
            let t242 = t39 * t45 * t173;
            let t245 = f64x8::splat(0.008185416666666667) * t238 * t82 + f64x8::splat(0.03190511708333333) * t38 * t242 + t234;
            let t247 = -t161 * t245 + t235 * t86;
            let t248 = t139 * t247;
            let t252 = ((t14).select(f64x8::splat(0.0), -t22 * t32 * t248 / f64x8::splat(9.0)));
            let t256 = ((t97).select(f64x8::splat(0.0), -t22 * t103 * t248 / f64x8::splat(9.0)));
            let t258 = ((t109).select(f64x8::splat(0.0), t252 + t256));
            let t263 = t208 * t258;
            let t265 = t116 * t258;
            let t267 = t119 * t258;
            let t269 = -f64x8::splat(126.32610668700724) * t263 - f64x8::splat(45.098196274873125) * t265 - f64x8::splat(3.577529259576) * t267;
            let t274 = ((t4).select(f64x8::splat(0.0), f64x8::splat(0.90165771) * t10 * t121 * t258 * t124 + f64x8::splat(0.25) * t10 * t206 * t269));
            let tvsigma0 = v_rho * t274;
            acc_vsigma = tvsigma0;
            let t276 = t9 * t121;
            let t277 = t199 * t124;
            let t283 = t199 * t199;
            let t288 = t28 * t28;
            let t291 = f64x8::splat(1.0) / t30 / t288 / t41;
            let t298 = t20 * t21 * t23;
            let t299 = t132 * t139;
            let t300 = t28 * t179;
            let t305 = f64x8::splat(1.0) / t138 / t88;
            let t306 = t179 * t179;
            let t307 = t305 * t306;
            let t311 = t62 * t148;
            let t313 = f64x8::splat(1.0) / t42 / t311;
            let t314 = t23 * t313;
            let t315 = t314 * t49;
            let t318 = t34 * t34;
            let t319 = f64x8::splat(1.0) / t318;
            let t320 = t60 * v_sigma;
            let t321 = t319 * t320;
            let t322 = t62 * t62;
            let t323 = t322 * t41;
            let t324 = f64x8::splat(1.0) / t323;
            let t329 = f64x8::splat(1.0) / t43 / t62;
            let t335 = f64x8::splat(0.0004691358024691358) * t59 * t61 * t313;
            let t336 = f64x8::splat(1.2566666666666666) * t140 * t315 - f64x8::splat(18.617283950617285) * t321 * t324 * t49 + f64x8::splat(11.0) / f64x8::splat(27.0) * t53 * t40 * t329 - t335;
            let t338 = t157 * t160;
            let t342 = f64x8::splat(1.0) / t159 / t85;
            let t343 = t69 * t342;
            let t344 = t177 * t177;
            let t350 = t23 / t42 / t148 * t81;
            let t354 = t39 * t329 * t173;
            let t358 = f64x8::splat(1.0) / t172 / t171;
            let t359 = t314 * t358;
            let t362 = f64x8::splat(0.05093148148148148) * t73 * t350 + f64x8::splat(0.4254015611111111) * t168 * t354 - f64x8::splat(0.5744942144582124) * t140 * t359 + t335;
            let t364 = -t161 * t362 - f64x8::splat(2.0) * t338 * t177 + t336 * t86 + f64x8::splat(2.0) * t343 * t344;
            let t365 = t139 * t364;
            let t370 = ((t14).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(81.0) * t22 * t23 * t291 * t89 * t288 + f64x8::splat(2.0) / f64x8::splat(27.0) * t298 * t299 * t300 + f64x8::splat(2.0) / f64x8::splat(9.0) * t22 * t32 * t307 - t22 * t32 * t365 / f64x8::splat(9.0)));
            let t371 = t99 * t99;
            let t374 = f64x8::splat(1.0) / t101 / t371 / t41;
            let t380 = t187 * t139;
            let t381 = t99 * t179;
            let t392 = ((t97).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(81.0) * t22 * t23 * t374 * t89 * t371 + f64x8::splat(2.0) / f64x8::splat(27.0) * t298 * t380 * t381 + f64x8::splat(2.0) / f64x8::splat(9.0) * t22 * t103 * t307 - t22 * t103 * t365 / f64x8::splat(9.0)));
            let t394 = ((t109).select(f64x8::splat(0.0), t370 + t392));
            let t399 = t10 * t121;
            let t400 = t199 * t205;
            let t401 = t400 * t215;
            let t405 = f64x8::splat(1.0) / t204 / t123;
            let t406 = t113 * t405;
            let t407 = t215 * t215;
            let t412 = f64x8::splat(1.0) / t115 / t114;
            let t413 = t412 * t283;
            let t417 = t208 * t283;
            let t425 = f64x8::splat(631.6305334350362) * t413 - f64x8::splat(126.32610668700724) * t208 * t394 + f64x8::splat(180.3927850994925) * t417 - f64x8::splat(45.098196274873125) * t116 * t394 + f64x8::splat(10.732587778728) * t116 * t283 - f64x8::splat(3.577529259576) * t119 * t394;
            let t430 = ((t4).select(f64x8::splat(0.0), f64x8::splat(1.80331542) * t276 * t277 + f64x8::splat(0.5) * t128 * t205 * t215 - f64x8::splat(1.80331542) * t10 * t119 * t283 * t124 + f64x8::splat(0.90165771) * t10 * t121 * t394 * t124 - f64x8::splat(1.80331542) * t399 * t401 - f64x8::splat(0.5) * t10 * t406 * t407 + f64x8::splat(0.25) * t10 * t206 * t425));
            let tv2rho20 = v_rho * t430 + f64x8::splat(2.0) * t220;
            acc_v2rho2 = tv2rho20;
            let t432 = t258 * t124;
            let t435 = t10 * t119;
            let t436 = t432 * t199;
            let t439 = t247 * t28;
            let t443 = t31 * t305;
            let t444 = t247 * t179;
            let t452 = t322 * v_rho;
            let t453 = f64x8::splat(1.0) / t452;
            let t454 = t319 * t453;
            let t455 = t60 * t49;
            let t463 = f64x8::splat(0.00014814814814814815) * t59 * t231 * t143;
            let t464 = -f64x8::splat(0.41888888888888887) * t222 * t143 * t49 * v_sigma + f64x8::splat(6.981481481481482) * t454 * t455 - t52 * t227 * t150 / f64x8::splat(9.0) + t463;
            let t466 = t235 * t160;
            let t469 = t245 * t177;
            let t480 = -f64x8::splat(0.01091388888888889) * t238 * t165 - f64x8::splat(0.12762046833333332) * t38 * t174 + f64x8::splat(0.21543533042182963) * t222 * t143 * t358 * v_sigma - t463;
            let t482 = -t161 * t480 - t466 * t177 - t338 * t245 + f64x8::splat(2.0) * t343 * t469 + t464 * t86;
            let t483 = t139 * t482;
            let t488 = ((t14).select(f64x8::splat(0.0), t298 * t299 * t439 / f64x8::splat(27.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t298 * t443 * t444 - t22 * t32 * t483 / f64x8::splat(9.0)));
            let t489 = t247 * t99;
            let t493 = t102 * t305;
            let t501 = ((t97).select(f64x8::splat(0.0), t298 * t380 * t489 / f64x8::splat(27.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t298 * t493 * t444 - t22 * t103 * t483 / f64x8::splat(9.0)));
            let t503 = ((t109).select(f64x8::splat(0.0), t488 + t501));
            let t508 = t258 * t205;
            let t509 = t508 * t215;
            let t515 = t400 * t269;
            let t518 = t10 * t113;
            let t519 = t405 * t269;
            let t520 = t519 * t215;
            let t523 = t412 * t258;
            let t526 = t208 * t503;
            let t530 = t116 * t503;
            let t536 = f64x8::splat(631.6305334350362) * t523 * t199 - f64x8::splat(126.32610668700724) * t526 + f64x8::splat(180.3927850994925) * t263 * t199 - f64x8::splat(45.098196274873125) * t530 + f64x8::splat(10.732587778728) * t265 * t199 - f64x8::splat(3.577529259576) * t119 * t503;
            let t541 = ((t4).select(f64x8::splat(0.0), f64x8::splat(0.90165771) * t276 * t432 - f64x8::splat(1.80331542) * t435 * t436 + f64x8::splat(0.90165771) * t10 * t121 * t503 * t124 - f64x8::splat(0.90165771) * t399 * t509 + f64x8::splat(0.25) * t128 * t205 * t269 - f64x8::splat(0.90165771) * t399 * t515 - f64x8::splat(0.5) * t518 * t520 + f64x8::splat(0.25) * t10 * t206 * t536));
            let tv2rhosigma0 = v_rho * t541 + t274;
            acc_v2rhosigma = tv2rhosigma0;
            let t543 = t258 * t258;
            let t548 = t247 * t247;
            let t549 = t305 * t548;
            let t553 = f64x8::splat(1.0) / t322;
            let t554 = t319 * t553;
            let t555 = t49 * v_sigma;
            let t558 = t23 * t65;
            let t563 = f64x8::splat(2.777777777777778e-05) * t59 * t558;
            let t564 = -f64x8::splat(2.6180555555555554) * t554 * t555 + f64x8::splat(0.10472222222222222) * t59 * t558 * t49 - t563;
            let t568 = t245 * t245;
            let t573 = t71 / t72 / v_sigma;
            let t576 = f64x8::splat(1.0) / v_sigma;
            let t577 = t38 * t576;
            let t580 = t558 * t358;
            let t583 = -f64x8::splat(0.004092708333333334) * t573 * t82 + f64x8::splat(0.015952558541666665) * t577 * t242 - f64x8::splat(0.08078824890818612) * t59 * t580 + t563;
            let t585 = -t161 * t583 - f64x8::splat(2.0) * t466 * t245 + f64x8::splat(2.0) * t343 * t568 + t564 * t86;
            let t586 = t139 * t585;
            let t591 = ((t14).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(9.0) * t22 * t32 * t549 - t22 * t32 * t586 / f64x8::splat(9.0)));
            let t599 = ((t97).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(9.0) * t22 * t103 * t549 - t22 * t103 * t586 / f64x8::splat(9.0)));
            let t601 = ((t109).select(f64x8::splat(0.0), t591 + t599));
            let t606 = t508 * t269;
            let t609 = t269 * t269;
            let t613 = t412 * t543;
            let t615 = t208 * t601;
            let t617 = t208 * t543;
            let t619 = t116 * t601;
            let t625 = f64x8::splat(631.6305334350362) * t613 - f64x8::splat(126.32610668700724) * t615 + f64x8::splat(180.3927850994925) * t617 - f64x8::splat(45.098196274873125) * t619 + f64x8::splat(10.732587778728) * t116 * t543 - f64x8::splat(3.577529259576) * t119 * t601;
            let t630 = ((t4).select(f64x8::splat(0.0), -f64x8::splat(1.80331542) * t10 * t119 * t543 * t124 + f64x8::splat(0.90165771) * t10 * t121 * t601 * t124 - f64x8::splat(1.80331542) * t399 * t606 - f64x8::splat(0.5) * t10 * t406 * t609 + f64x8::splat(0.25) * t10 * t206 * t625));
            let tv2sigma20 = v_rho * t630;
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
