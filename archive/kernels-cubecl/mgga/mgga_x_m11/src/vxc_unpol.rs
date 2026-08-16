//! MGGA_X_M11 vxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_m11.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_x_m11_vxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    vlapl: &mut Array<f64>,
    vtau: &mut Array<f64>,
    param_a_0: f64,
    param_a_1: f64,
    param_a_2: f64,
    param_a_3: f64,
    param_a_4: f64,
    param_a_5: f64,
    param_a_6: f64,
    param_a_7: f64,
    param_a_8: f64,
    param_a_9: f64,
    param_a_10: f64,
    param_a_11: f64,
    param_b_0: f64,
    param_b_1: f64,
    param_b_2: f64,
    param_b_3: f64,
    param_b_4: f64,
    param_b_5: f64,
    param_b_6: f64,
    param_b_7: f64,
    param_b_8: f64,
    param_b_9: f64,
    param_b_10: f64,
    param_b_11: f64,
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = M_CBRTPI;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5::<f64>(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t13 = t12 <= zeta_threshold;
        let t14 = pow_1_3::<f64>(zeta_threshold);
        let t16 = pow_1_3::<f64>(t12);
        let t18 = piecewise3::<f64>(t13, t14 * zeta_threshold, t16 * t12);
        let t19 = t4 / t5 * t18;
        let t20 = pow_1_3::<f64>(rho[ip]);
        let t21 = pow_1_3::<f64>(9.0);
        let t22 = t21 * t21;
        let t24 = pow_1_3::<f64>(1.0 / M_PI);
        let t25 = t24 * t24;
        let t27 = t22 * t25 * param_hyb_omega_0;
        let t30 = piecewise3::<f64>(t13, t14, t16);
        let t31 = 1.0 / t30;
        let t34 = t27 * t4 / t20 * t31 / 18.0;
        let t35 = 0.135e1 <= t34;
        let t36 = 0.135e1 < t34;
        let t37 = piecewise3::<f64>(t36, t34, 0.135e1);
        let t38 = t37 * t37;
        let t41 = t38 * t38;
        let t42 = 1.0 / t41;
        let t44 = t41 * t38;
        let t45 = 1.0 / t44;
        let t47 = t41 * t41;
        let t48 = 1.0 / t47;
        let t51 = 1.0 / t47 / t38;
        let t54 = 1.0 / t47 / t41;
        let t57 = 1.0 / t47 / t44;
        let t59 = t47 * t47;
        let t60 = 1.0 / t59;
        let t63 = piecewise3::<f64>(t36, 0.135e1, t34);
        let t64 = f64::sqrt(M_PI);
        let t65 = 1.0 / t63;
        let t67 = erf_approx::<f64>(t65 / 2.0);
        let t69 = t63 * t63;
        let t70 = 1.0 / t69;
        let t72 = f64::exp(-t70 / 4.0);
        let t73 = t72 - 1.0;
        let t76 = t72 - 3.0 / 2.0 - 2.0 * t69 * t73;
        let t79 = 2.0 * t63 * t76 + t64 * t67;
        let t83 = piecewise3::<f64>(t35, 1.0 / t38 / 36.0 - t42 / 960.0 + t45 / 26880.0 - t48 / 829440.0 + t51 / 28385280.0 - t54 / 0.107347968e10 + t57 / 0.445906944e11 - t60 / 0.20214448128e13, 1.0 - 8.0 / 3.0 * t63 * t79);
        let t84 = t20 * t83;
        let t85 = M_CBRT6;
        let t86 = M_PI * M_PI;
        let t87 = pow_1_3::<f64>(t86);
        let t88 = t87 * t87;
        let t89 = 1.0 / t88;
        let t90 = t85 * t89;
        let t91 = M_CBRT2;
        let t92 = t91 * t91;
        let t93 = sigma[ip] * t92;
        let t94 = rho[ip] * rho[ip];
        let t95 = t20 * t20;
        let t97 = 1.0 / t95 / t94;
        let t99 = t90 * t93 * t97;
        let t101 = 0.804e0 + 0.914625e-2 * t99;
        let t104 = 0.1804e1 - 0.646416e0 / t101;
        let t106 = param_a_1;
        let t107 = t85 * t85;
        let t109 = 3.0 / 10.0 * t107 * t88;
        let t110 = tau[ip] * t92;
        let t112 = 1.0 / t95 / rho[ip];
        let t113 = t110 * t112;
        let t114 = t109 - t113;
        let t115 = t106 * t114;
        let t116 = t109 + t113;
        let t117 = 1.0 / t116;
        let t119 = param_a_2;
        let t120 = t114 * t114;
        let t121 = t119 * t120;
        let t122 = t116 * t116;
        let t123 = 1.0 / t122;
        let t125 = param_a_3;
        let t126 = t120 * t114;
        let t127 = t125 * t126;
        let t128 = t122 * t116;
        let t129 = 1.0 / t128;
        let t131 = param_a_4;
        let t132 = t120 * t120;
        let t133 = t131 * t132;
        let t134 = t122 * t122;
        let t135 = 1.0 / t134;
        let t137 = param_a_5;
        let t138 = t132 * t114;
        let t139 = t137 * t138;
        let t140 = t134 * t116;
        let t141 = 1.0 / t140;
        let t143 = param_a_6;
        let t144 = t132 * t120;
        let t145 = t143 * t144;
        let t146 = t134 * t122;
        let t147 = 1.0 / t146;
        let t149 = param_a_7;
        let t150 = t132 * t126;
        let t151 = t149 * t150;
        let t152 = t134 * t128;
        let t153 = 1.0 / t152;
        let t155 = param_a_8;
        let t156 = t132 * t132;
        let t157 = t155 * t156;
        let t158 = t134 * t134;
        let t159 = 1.0 / t158;
        let t161 = param_a_9;
        let t162 = t156 * t114;
        let t163 = t161 * t162;
        let t165 = 1.0 / t158 / t116;
        let t167 = param_a_10;
        let t168 = t156 * t120;
        let t169 = t167 * t168;
        let t171 = 1.0 / t158 / t122;
        let t173 = param_a_11;
        let t174 = t156 * t126;
        let t175 = t173 * t174;
        let t177 = 1.0 / t158 / t128;
        let t179 = t115 * t117 + t121 * t123 + t127 * t129 + t133 * t135 + t139 * t141 + t145 * t147 + t151 * t153 + t157 * t159 + t163 * t165 + t169 * t171 + t175 * t177 + param_a_0;
        let t182 = f64::exp(-0.93189002206715572255e-2 * t99);
        let t184 = 0.1552e1 - 0.552e0 * t182;
        let t186 = param_b_1;
        let t187 = t186 * t114;
        let t189 = param_b_2;
        let t190 = t189 * t120;
        let t192 = param_b_3;
        let t193 = t192 * t126;
        let t195 = param_b_4;
        let t196 = t195 * t132;
        let t198 = param_b_5;
        let t199 = t198 * t138;
        let t201 = param_b_6;
        let t202 = t201 * t144;
        let t204 = param_b_7;
        let t205 = t204 * t150;
        let t207 = param_b_8;
        let t208 = t207 * t156;
        let t210 = param_b_9;
        let t211 = t210 * t162;
        let t213 = param_b_10;
        let t214 = t213 * t168;
        let t216 = param_b_11;
        let t217 = t216 * t174;
        let t219 = t187 * t117 + t190 * t123 + t193 * t129 + t196 * t135 + t199 * t141 + t202 * t147 + t205 * t153 + t208 * t159 + t211 * t165 + t214 * t171 + t217 * t177 + param_b_0;
        let t221 = t104 * t179 + t184 * t219;
        let t225 = piecewise3::<f64>(t3, 0.0, -3.0 / 8.0 * t19 * t84 * t221);
        let tzk0 = 2.0 * t225;
        zk[ip] += tzk0;
        let t226 = 1.0 / t95;
        let t227 = t226 * t83;
        let t231 = t38 * t37;
        let t232 = 1.0 / t231;
        let t238 = t27 * t4 / t20 / rho[ip] * t31 / 54.0;
        let t239 = piecewise3::<f64>(t36, -t238, 0.0);
        let t242 = t41 * t37;
        let t243 = 1.0 / t242;
        let t246 = t41 * t231;
        let t247 = 1.0 / t246;
        let t251 = 1.0 / t47 / t37;
        let t255 = 1.0 / t47 / t231;
        let t259 = 1.0 / t47 / t242;
        let t263 = 1.0 / t47 / t246;
        let t267 = 1.0 / t59 / t37;
        let t271 = piecewise3::<f64>(t36, 0.0, -t238);
        let t273 = t72 * t70;
        let t277 = t69 * t63;
        let t278 = 1.0 / t277;
        let t282 = t63 * t73;
        let t287 = t278 * t271 * t72 / 2.0 - 4.0 * t282 * t271 - t65 * t271 * t72;
        let t290 = -t273 * t271 + 2.0 * t271 * t76 + 2.0 * t63 * t287;
        let t294 = piecewise3::<f64>(t35, -t232 * t239 / 18.0 + t243 * t239 / 240.0 - t247 * t239 / 4480.0 + t251 * t239 / 103680.0 - t255 * t239 / 2838528.0 + t259 * t239 / 89456640.0 - t263 * t239 / 0.31850496e10 + t267 * t239 / 0.1263403008e12, -8.0 / 3.0 * t271 * t79 - 8.0 / 3.0 * t63 * t290);
        let t295 = t20 * t294;
        let t299 = t101 * t101;
        let t302 = 1.0 / t299 * t85 * t89;
        let t303 = t94 * rho[ip];
        let t305 = 1.0 / t95 / t303;
        let t310 = t106 * tau[ip];
        let t311 = t92 * t97;
        let t312 = t311 * t117;
        let t315 = t115 * t123;
        let t316 = t110 * t97;
        let t319 = t119 * t114;
        let t320 = t319 * t123;
        let t323 = t121 * t129;
        let t326 = t125 * t120;
        let t327 = t326 * t129;
        let t330 = t127 * t135;
        let t333 = t131 * t126;
        let t334 = t333 * t135;
        let t337 = t133 * t141;
        let t340 = t137 * t132;
        let t341 = t340 * t141;
        let t344 = t139 * t147;
        let t347 = t143 * t138;
        let t348 = t347 * t147;
        let t351 = 5.0 / 3.0 * t310 * t312 + 5.0 / 3.0 * t315 * t316 + 10.0 / 3.0 * t320 * t316 + 10.0 / 3.0 * t323 * t316 + 5.0 * t327 * t316 + 5.0 * t330 * t316 + 20.0 / 3.0 * t334 * t316 + 20.0 / 3.0 * t337 * t316 + 25.0 / 3.0 * t341 * t316 + 25.0 / 3.0 * t344 * t316 + 10.0 * t348 * t316;
        let t352 = t145 * t153;
        let t355 = t149 * t144;
        let t356 = t355 * t153;
        let t359 = t151 * t159;
        let t362 = t155 * t150;
        let t363 = t362 * t159;
        let t366 = t157 * t165;
        let t369 = t161 * t156;
        let t370 = t369 * t165;
        let t373 = t163 * t171;
        let t376 = t167 * t162;
        let t377 = t376 * t171;
        let t380 = t169 * t177;
        let t383 = t173 * t168;
        let t384 = t383 * t177;
        let t388 = 1.0 / t158 / t134;
        let t389 = t175 * t388;
        let t392 = 10.0 * t352 * t316 + 35.0 / 3.0 * t356 * t316 + 35.0 / 3.0 * t359 * t316 + 40.0 / 3.0 * t363 * t316 + 40.0 / 3.0 * t366 * t316 + 15.0 * t370 * t316 + 15.0 * t373 * t316 + 50.0 / 3.0 * t377 * t316 + 50.0 / 3.0 * t380 * t316 + 55.0 / 3.0 * t384 * t316 + 55.0 / 3.0 * t389 * t316;
        let t393 = t351 + t392;
        let t395 = t90 * sigma[ip];
        let t396 = t92 * t305;
        let t397 = t182 * t219;
        let t401 = t186 * tau[ip];
        let t404 = t187 * t123;
        let t407 = t189 * t114;
        let t408 = t407 * t123;
        let t411 = t190 * t129;
        let t414 = t192 * t120;
        let t415 = t414 * t129;
        let t418 = t193 * t135;
        let t421 = t195 * t126;
        let t422 = t421 * t135;
        let t425 = t196 * t141;
        let t428 = t198 * t132;
        let t429 = t428 * t141;
        let t432 = t199 * t147;
        let t435 = t201 * t138;
        let t436 = t435 * t147;
        let t439 = 5.0 / 3.0 * t401 * t312 + 5.0 / 3.0 * t404 * t316 + 10.0 / 3.0 * t408 * t316 + 10.0 / 3.0 * t411 * t316 + 5.0 * t415 * t316 + 5.0 * t418 * t316 + 20.0 / 3.0 * t422 * t316 + 20.0 / 3.0 * t425 * t316 + 25.0 / 3.0 * t429 * t316 + 25.0 / 3.0 * t432 * t316 + 10.0 * t436 * t316;
        let t440 = t202 * t153;
        let t443 = t204 * t144;
        let t444 = t443 * t153;
        let t447 = t205 * t159;
        let t450 = t207 * t150;
        let t451 = t450 * t159;
        let t454 = t208 * t165;
        let t457 = t210 * t156;
        let t458 = t457 * t165;
        let t461 = t211 * t171;
        let t464 = t213 * t162;
        let t465 = t464 * t171;
        let t468 = t214 * t177;
        let t471 = t216 * t168;
        let t472 = t471 * t177;
        let t475 = t217 * t388;
        let t478 = 10.0 * t440 * t316 + 35.0 / 3.0 * t444 * t316 + 35.0 / 3.0 * t447 * t316 + 40.0 / 3.0 * t451 * t316 + 40.0 / 3.0 * t454 * t316 + 15.0 * t458 * t316 + 15.0 * t461 * t316 + 50.0 / 3.0 * t465 * t316 + 50.0 / 3.0 * t468 * t316 + 55.0 / 3.0 * t472 * t316 + 55.0 / 3.0 * t475 * t316;
        let t479 = t439 + t478;
        let t481 = -0.1576608624e-1 * t302 * t93 * t305 * t179 + t104 * t393 - 0.13717421124828532236e-1 * t395 * t396 * t397 + t184 * t479;
        let t486 = piecewise3::<f64>(t3, 0.0, -t19 * t227 * t221 / 8.0 - 3.0 / 8.0 * t19 * t295 * t221 - 3.0 / 8.0 * t19 * t84 * t481);
        let tvrho0 = 2.0 * rho[ip] * t486 + 2.0 * t225;
        vrho[ip] += tvrho0;
        let t492 = t90 * t92;
        let t493 = t97 * t182;
        let t497 = 0.591228234e-2 * t302 * t311 * t179 + 0.51440329218106995885e-2 * t492 * t493 * t219;
        let t501 = piecewise3::<f64>(t3, 0.0, -3.0 / 8.0 * t19 * t84 * t497);
        let tvsigma0 = 2.0 * rho[ip] * t501;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t503 = t106 * t92;
        let t504 = t112 * t117;
        let t506 = t123 * t92;
        let t507 = t506 * t112;
        let t511 = t129 * t92;
        let t512 = t511 * t112;
        let t517 = t135 * t92;
        let t518 = t517 * t112;
        let t523 = t141 * t92;
        let t524 = t523 * t112;
        let t529 = t147 * t92;
        let t530 = t529 * t112;
        let t535 = -t115 * t507 - 2.0 * t121 * t512 - 3.0 * t127 * t518 - 4.0 * t133 * t524 - 5.0 * t139 * t530 - 2.0 * t319 * t507 - 3.0 * t326 * t512 - 4.0 * t333 * t518 - 5.0 * t340 * t524 - 6.0 * t347 * t530 - t503 * t504;
        let t536 = t153 * t92;
        let t537 = t536 * t112;
        let t542 = t159 * t92;
        let t543 = t542 * t112;
        let t548 = t165 * t92;
        let t549 = t548 * t112;
        let t554 = t171 * t92;
        let t555 = t554 * t112;
        let t560 = t177 * t92;
        let t561 = t560 * t112;
        let t566 = t388 * t92;
        let t567 = t566 * t112;
        let t570 = -6.0 * t145 * t537 - 7.0 * t151 * t543 - 8.0 * t157 * t549 - 9.0 * t163 * t555 - 10.0 * t169 * t561 - 11.0 * t175 * t567 - 7.0 * t355 * t537 - 8.0 * t362 * t543 - 9.0 * t369 * t549 - 10.0 * t376 * t555 - 11.0 * t383 * t561;
        let t571 = t535 + t570;
        let t573 = t186 * t92;
        let t594 = -t187 * t507 - 2.0 * t190 * t512 - 3.0 * t193 * t518 - 4.0 * t196 * t524 - 5.0 * t199 * t530 - 2.0 * t407 * t507 - 3.0 * t414 * t512 - 4.0 * t421 * t518 - 5.0 * t428 * t524 - 6.0 * t435 * t530 - t573 * t504;
        let t617 = -6.0 * t202 * t537 - 7.0 * t205 * t543 - 8.0 * t208 * t549 - 9.0 * t211 * t555 - 10.0 * t214 * t561 - 11.0 * t217 * t567 - 7.0 * t443 * t537 - 8.0 * t450 * t543 - 9.0 * t457 * t549 - 10.0 * t464 * t555 - 11.0 * t471 * t561;
        let t618 = t594 + t617;
        let t620 = t104 * t571 + t184 * t618;
        let t624 = piecewise3::<f64>(t3, 0.0, -3.0 / 8.0 * t19 * t84 * t620);
        let tvtau0 = 2.0 * rho[ip] * t624;
        vtau[ip] += tvtau0;
    }
}
