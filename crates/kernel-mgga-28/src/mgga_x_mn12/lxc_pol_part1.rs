//! MGGA_X_MN12 lxc pol kernel — split part 1/2 (v4sigmatau3_0_v4sigmatau3_1_v4sigmatau3_2_v4sigmatau3_3_v4sigmatau3_4_v4sigmatau3_5_v4sigmatau3_6_v4sigmatau3_7_v4sigmatau3_8_v4sigmatau3_9_v4sigmatau3_10_v4sigmatau3_11_v4lapl4_0_v4lapl4_1_v4lapl4_2_v4lapl4_3_v4lapl4_4_v4lapl3tau_0_v4lapl3tau_1_v4lapl3tau_2_v4lapl3tau_3_v4lapl3tau_4_v4lapl3tau_5_v4lapl3tau_6_v4lapl3tau_7_v4lapl2tau2_0_v4lapl2tau2_1_v4lapl2tau2_2_v4lapl2tau2_3_v4lapl2tau2_4_v4lapl2tau2_5_v4lapl2tau2_6_v4lapl2tau2_7_v4lapl2tau2_8_v4lapltau3_0_v4lapltau3_1_v4lapltau3_2_v4lapltau3_3_v4lapltau3_4_v4lapltau3_5_v4lapltau3_6_v4lapltau3_7_v4tau4_0_v4tau4_1_v4tau4_2_v4tau4_3_v4tau4_4).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_mn12.c`.
//! Split sub-kernel: outputs [v4sigmatau3, v4sigmatau3, v4sigmatau3, v4sigmatau3, v4sigmatau3, v4sigmatau3, v4sigmatau3, v4sigmatau3, v4sigmatau3, v4sigmatau3, v4sigmatau3, v4sigmatau3, v4lapl4, v4lapl4, v4lapl4, v4lapl4, v4lapl4, v4lapl3tau, v4lapl3tau, v4lapl3tau, v4lapl3tau, v4lapl3tau, v4lapl3tau, v4lapl3tau, v4lapl3tau, v4lapl2tau2, v4lapl2tau2, v4lapl2tau2, v4lapl2tau2, v4lapl2tau2, v4lapl2tau2, v4lapl2tau2, v4lapl2tau2, v4lapl2tau2, v4lapltau3, v4lapltau3, v4lapltau3, v4lapltau3, v4lapltau3, v4lapltau3, v4lapltau3, v4lapltau3, v4tau4, v4tau4, v4tau4, v4tau4, v4tau4] (503 lines).

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_x_mn12_lxc_pol_part1(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    v4sigmatau3: &mut Array<f64>,
    v4lapl4: &mut Array<f64>,
    v4lapl3tau: &mut Array<f64>,
    v4lapl2tau2: &mut Array<f64>,
    v4lapltau3: &mut Array<f64>,
    v4tau4: &mut Array<f64>,
    param_c_1: f64,
    param_c_2: f64,
    param_c_3: f64,
    param_c_4: f64,
    param_c_5: f64,
    param_c_7: f64,
    param_c_8: f64,
    param_c_9: f64,
    param_c_10: f64,
    param_c_12: f64,
    param_c_13: f64,
    param_c_14: f64,
    param_c_16: f64,
    param_c_17: f64,
    param_c_19: f64,
    param_c_20: f64,
    param_c_21: f64,
    param_c_22: f64,
    param_c_24: f64,
    param_c_25: f64,
    param_c_26: f64,
    param_c_28: f64,
    param_c_29: f64,
    param_c_31: f64,
    param_c_32: f64,
    param_c_33: f64,
    param_c_35: f64,
    param_c_36: f64,
    param_c_38: f64,
    param_c_39: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < v4sigmatau3.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let lapl0 = lapl[ip * 2];
        let lapl1 = lapl[ip * 2 + 1];
        let tau0 = tau[ip * 2];
        let tau1 = tau[ip * 2 + 1];
        let t2 = rho0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
        let t7 = rho0 + rho1;
        let t8 = 1.0 / t7;
        let t11 = 2.0 * rho0 * t8 <= zeta_threshold;
        let t12 = zeta_threshold - 1.0;
        let t15 = 2.0 * rho1 * t8 <= zeta_threshold;
        let t16 = -t12;
        let t17 = rho0 - rho1;
        let t18 = t17 * t8;
        let t19 = piecewise5(t11, t12, t15, t16, t18);
        let t20 = 1.0 + t19;
        let t21 = t20 <= zeta_threshold;
        let t22 = pow_1_3(zeta_threshold);
        let t23 = t22 * zeta_threshold;
        let t24 = pow_1_3(t20);
        let t26 = piecewise3(t21, t23, t24 * t20);
        let t27 = pow_1_3(t7);
        let t28 = t26 * t27;
        let t31 = M_CBRT6;
        let t32 = t31 * t31;
        let t33 = M_PI * M_PI;
        let t34 = pow_1_3(t33);
        let t35 = t34 * t34;
        let t37 = 3.0 / 10.0 * t32 * t35;
        let t38 = pow_1_3(rho0);
        let t39 = t38 * t38;
        let t41 = 1.0 / t39 / rho0;
        let t42 = tau0 * t41;
        let t43 = t37 - t42;
        let t45 = t37 + t42;
        let t49 = t43 * t43;
        let t51 = t45 * t45;
        let t55 = t49 * t43;
        let t57 = t51 * t45;
        let t58 = 1.0 / t57;
        let t61 = t49 * t49;
        let t63 = t51 * t51;
        let t64 = 1.0 / t63;
        let t70 = 1.0 / t63 / t45;
        let t73 = param_c_7;
        let t74 = t73 * t43;
        let t76 = param_c_8;
        let t77 = t76 * t49;
        let t79 = param_c_9;
        let t80 = t79 * t55;
        let t82 = param_c_10;
        let t83 = t82 * t61;
        let t87 = rho0 * rho0;
        let t89 = 1.0 / t39 / t87;
        let t92 = 1.0 + 0.4e-2 * sigma0 * t89;
        let t93 = 1.0 / t92;
        let t98 = param_c_12;
        let t99 = t98 * t43;
        let t101 = param_c_13;
        let t102 = t101 * t49;
        let t104 = param_c_14;
        let t105 = t104 * t55;
        let t108 = sigma0 * sigma0;
        let t110 = t87 * t87;
        let t111 = t110 * rho0;
        let t113 = 1.0 / t38 / t111;
        let t114 = t92 * t92;
        let t115 = 1.0 / t114;
        let t116 = t113 * t115;
        let t120 = param_c_16;
        let t121 = t120 * t43;
        let t123 = param_c_17;
        let t124 = t123 * t49;
        let t127 = t108 * sigma0;
        let t129 = t110 * t110;
        let t130 = 1.0 / t129;
        let t131 = t114 * t92;
        let t132 = 1.0 / t131;
        let t133 = t130 * t132;
        let t151 = M_CBRT2;
        let t152 = 1.0 / t27 * t151;
        let t154 = 1.0 + t18 <= zeta_threshold;
        let t156 = 1.0 - t18 <= zeta_threshold;
        let t157 = piecewise5(t154, t12, t156, t16, t18);
        let t158 = 1.0 + t157;
        let t159 = 1.0 / t158;
        let t160 = pow_1_3(t159);
        let t163 = 1.0 + 0.39999999999999999998e0 * t152 * t160;
        let t164 = 1.0 / t163;
        let t167 = param_c_24;
        let t168 = t167 * t43;
        let t170 = param_c_25;
        let t171 = t170 * t49;
        let t173 = param_c_26;
        let t174 = t173 * t55;
        let t182 = param_c_28;
        let t183 = t182 * t43;
        let t185 = param_c_29;
        let t186 = t185 * t49;
        let t190 = t116 * t164;
        let t204 = t163 * t163;
        let t205 = 1.0 / t204;
        let t208 = param_c_35;
        let t209 = t208 * t43;
        let t211 = param_c_36;
        let t212 = t211 * t49;
        let t441 = t93 * t205;
        let t473 = t76 * t43;
        let t480 = t79 * t49;
        let t487 = t82 * t55;
        let t506 = t101 * t43;
        let t511 = t104 * t49;
        let t528 = t123 * t43;
        let t542 = t114 * t114;
        let t543 = 1.0 / t542;
        let t561 = t185 * t43;
        let t580 = t211 * t43;
        let t598 = t170 * t43;
        let t603 = t173 * t49;
        let t629 = 1.0 / t63 / t51;
        let t1101 = t129 * t87;
        let t1103 = 1.0 / t39 / t1101;
        let t1775 = t173 * t43;
        let t1831 = 1.0 / t63 / t57;
        let t1904 = t104 * t43;
        let t2002 = t79 * t43;
        let t2013 = t82 * t49;
        let t5190 = t82 * t43;
        let t1104 = t1103 * t543;
        let t1108 = t93 * t164;
        let t1116 = t133 * t164;
        let t1122 = t116 * t205;
        let t12283 = 1.0 / t111;
        let t12287 = t64 * t12283;
        let t12295 = t70 * t12283;
        let t12305 = t629 * t12283;
        let t12308 = -6.0 * t167 * t12283 * t58 - 12.0 * t170 * t12283 * t58 - 6.0 * t173 * t12283 * t58 - 6.0 * t168 * t12287 - 54.0 * t1775 * t12287 - 36.0 * t598 * t12287 - 24.0 * t171 * t12295 - 108.0 * t603 * t12295 - 60.0 * t174 * t12305;
        let t12309 = t12308 * sigma0;
        let t12324 = -6.0 * t182 * t12283 * t58 - 12.0 * t185 * t12283 * t58 - 6.0 * t183 * t12287 - 36.0 * t561 * t12287 - 24.0 * t186 * t12295;
        let t12325 = t12324 * t108;
        let t12340 = -6.0 * t208 * t12283 * t58 - 12.0 * t211 * t12283 * t58 - 6.0 * t209 * t12287 - 36.0 * t580 * t12287 - 24.0 * t212 * t12295;
        let t12341 = t12340 * sigma0;
        let t12371 = t1831 * t12283;
        let t12374 = -6.0 * t73 * t12283 * t58 - 12.0 * t76 * t12283 * t58 - 6.0 * t79 * t12283 * t58 - 54.0 * t2002 * t12287 - 36.0 * t473 * t12287 - 24.0 * t5190 * t12287 - 6.0 * t74 * t12287 - 144.0 * t2013 * t12295 - 108.0 * t480 * t12295 - 24.0 * t77 * t12295 - 240.0 * t487 * t12305 - 60.0 * t80 * t12305 - 120.0 * t83 * t12371;
        let t12375 = t12374 * sigma0;
        let t12419 = -12.0 * t101 * t12283 * t58 - 6.0 * t104 * t12283 * t58 - 6.0 * t98 * t12283 * t58 - 24.0 * t102 * t12295 - 60.0 * t105 * t12305 - 54.0 * t1904 * t12287 - 36.0 * t506 * t12287 - 6.0 * t99 * t12287 - 108.0 * t511 * t12295;
        let t12420 = t12419 * t108;
        let t12435 = -6.0 * t120 * t12283 * t58 - 12.0 * t123 * t12283 * t58 - 6.0 * t121 * t12287 - 36.0 * t528 * t12287 - 24.0 * t124 * t12295;
        let t12436 = t12435 * t127;
        let t29572 = 0.32e-4 * t12419 * sigma0 * t116 - 0.128e-6 * t12420 * t133 + 0.192e-6 * t12435 * t108 * t133 - 0.768e-9 * t12436 * t1104 + 0.4e-2 * t12374 * t89 * t93 - 0.16e-4 * t12375 * t116 + 0.4e-2 * t12308 * t89 * t1108 - 0.16e-4 * t12309 * t190 + 0.32e-4 * t12324 * sigma0 * t190 - 0.128e-6 * t12325 * t1116 + 0.4e-2 * t12340 * t89 * t441 - 0.16e-4 * t12341 * t1122;
        let t29576 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t28 * t29572);
        let tv4sigmatau30 = t7 * t29576;
        v4sigmatau3[ip * 12] += tv4sigmatau30;
        let tv4sigmatau31 = 0.0;
        v4sigmatau3[ip * 12 + 1] += tv4sigmatau31;
        let tv4sigmatau32 = 0.0;
        v4sigmatau3[ip * 12 + 2] += tv4sigmatau32;
        let tv4sigmatau33 = 0.0;
        v4sigmatau3[ip * 12 + 3] += tv4sigmatau33;
        let tv4sigmatau34 = 0.0;
        v4sigmatau3[ip * 12 + 4] += tv4sigmatau34;
        let tv4sigmatau35 = 0.0;
        v4sigmatau3[ip * 12 + 5] += tv4sigmatau35;
        let tv4sigmatau36 = 0.0;
        v4sigmatau3[ip * 12 + 6] += tv4sigmatau36;
        let tv4sigmatau37 = 0.0;
        v4sigmatau3[ip * 12 + 7] += tv4sigmatau37;
        let tv4sigmatau38 = 0.0;
        v4sigmatau3[ip * 12 + 8] += tv4sigmatau38;
        let tv4sigmatau39 = 0.0;
        v4sigmatau3[ip * 12 + 9] += tv4sigmatau39;
        let tv4sigmatau310 = 0.0;
        v4sigmatau3[ip * 12 + 10] += tv4sigmatau310;
        let t235 = rho1 <= dens_threshold;
        let t236 = -t17;
        let t238 = piecewise5(t15, t12, t11, t16, t236 * t8);
        let t239 = 1.0 + t238;
        let t240 = t239 <= zeta_threshold;
        let t241 = pow_1_3(t239);
        let t243 = piecewise3(t240, t23, t241 * t239);
        let t244 = t243 * t27;
        let t245 = pow_1_3(rho1);
        let t246 = t245 * t245;
        let t248 = 1.0 / t246 / rho1;
        let t249 = tau1 * t248;
        let t250 = t37 - t249;
        let t252 = t37 + t249;
        let t255 = t250 * t250;
        let t257 = t252 * t252;
        let t260 = t255 * t250;
        let t262 = t257 * t252;
        let t263 = 1.0 / t262;
        let t265 = t255 * t255;
        let t267 = t257 * t257;
        let t268 = 1.0 / t267;
        let t273 = 1.0 / t267 / t252;
        let t275 = t73 * t250;
        let t277 = t76 * t255;
        let t279 = t79 * t260;
        let t281 = t82 * t265;
        let t285 = rho1 * rho1;
        let t287 = 1.0 / t246 / t285;
        let t290 = 1.0 + 0.4e-2 * sigma2 * t287;
        let t291 = 1.0 / t290;
        let t295 = t98 * t250;
        let t297 = t101 * t255;
        let t299 = t104 * t260;
        let t302 = sigma2 * sigma2;
        let t304 = t285 * t285;
        let t305 = t304 * rho1;
        let t307 = 1.0 / t245 / t305;
        let t308 = t290 * t290;
        let t309 = 1.0 / t308;
        let t310 = t307 * t309;
        let t313 = t120 * t250;
        let t315 = t123 * t255;
        let t318 = t302 * sigma2;
        let t320 = t304 * t304;
        let t321 = 1.0 / t320;
        let t322 = t308 * t290;
        let t323 = 1.0 / t322;
        let t324 = t321 * t323;
        let t336 = piecewise5(t156, t12, t154, t16, -t18);
        let t337 = 1.0 + t336;
        let t338 = 1.0 / t337;
        let t339 = pow_1_3(t338);
        let t342 = 1.0 + 0.39999999999999999998e0 * t152 * t339;
        let t343 = 1.0 / t342;
        let t345 = t167 * t250;
        let t347 = t170 * t255;
        let t349 = t173 * t260;
        let t356 = t182 * t250;
        let t358 = t185 * t255;
        let t362 = t310 * t343;
        let t372 = t342 * t342;
        let t373 = 1.0 / t372;
        let t375 = t208 * t250;
        let t377 = t211 * t255;
        let t739 = t291 * t373;
        let t820 = t170 * t250;
        let t827 = t173 * t255;
        let t843 = t185 * t250;
        let t890 = 1.0 / t267 / t257;
        let t916 = t211 * t250;
        let t1005 = t76 * t250;
        let t1010 = t79 * t255;
        let t1015 = t82 * t260;
        let t1032 = t101 * t250;
        let t1037 = t104 * t255;
        let t1054 = t123 * t250;
        let t1068 = t308 * t308;
        let t1069 = 1.0 / t1068;
        let t1143 = t320 * t285;
        let t1145 = 1.0 / t246 / t1143;
        let t2811 = t173 * t250;
        let t2835 = 1.0 / t267 / t262;
        let t2876 = t79 * t250;
        let t2887 = t82 * t255;
        let t2929 = t104 * t250;
        let t7284 = t82 * t250;
        let t1146 = t1145 * t1069;
        let t1150 = t291 * t343;
        let t1158 = t324 * t343;
        let t1164 = t310 * t373;
        let t12532 = 1.0 / t305;
        let t12536 = t268 * t12532;
        let t12544 = t273 * t12532;
        let t12547 = -6.0 * t182 * t12532 * t263 - 12.0 * t185 * t12532 * t263 - 6.0 * t356 * t12536 - 36.0 * t843 * t12536 - 24.0 * t358 * t12544;
        let t12548 = t12547 * t302;
        let t12563 = -6.0 * t208 * t12532 * t263 - 12.0 * t211 * t12532 * t263 - 6.0 * t375 * t12536 - 36.0 * t916 * t12536 - 24.0 * t377 * t12544;
        let t12564 = t12563 * sigma2;
        let t12586 = t890 * t12532;
        let t12589 = -6.0 * t167 * t12532 * t263 - 12.0 * t170 * t12532 * t263 - 6.0 * t173 * t12532 * t263 - 54.0 * t2811 * t12536 - 6.0 * t345 * t12536 - 36.0 * t820 * t12536 - 24.0 * t347 * t12544 - 108.0 * t827 * t12544 - 60.0 * t349 * t12586;
        let t12590 = t12589 * sigma2;
        let t12620 = -12.0 * t101 * t12532 * t263 - 6.0 * t104 * t12532 * t263 - 6.0 * t98 * t12532 * t263 - 36.0 * t1032 * t12536 - 108.0 * t1037 * t12544 - 54.0 * t2929 * t12536 - 6.0 * t295 * t12536 - 24.0 * t297 * t12544 - 60.0 * t299 * t12586;
        let t12621 = t12620 * t302;
        let t12636 = -6.0 * t120 * t12532 * t263 - 12.0 * t123 * t12532 * t263 - 36.0 * t1054 * t12536 - 6.0 * t313 * t12536 - 24.0 * t315 * t12544;
        let t12637 = t12636 * t318;
        let t12640 = t2835 * t12532;
        let t12694 = -6.0 * t73 * t12532 * t263 - 12.0 * t76 * t12532 * t263 - 6.0 * t79 * t12532 * t263 - 36.0 * t1005 * t12536 - 108.0 * t1010 * t12544 - 240.0 * t1015 * t12586 - 6.0 * t275 * t12536 - 54.0 * t2876 * t12536 - 24.0 * t7284 * t12536 - 24.0 * t277 * t12544 - 144.0 * t2887 * t12544 - 60.0 * t279 * t12586 - 120.0 * t281 * t12640;
        let t12695 = t12694 * sigma2;
        let t29607 = 0.4e-2 * t12694 * t287 * t291 - 0.16e-4 * t12695 * t310 + 0.32e-4 * t12620 * sigma2 * t310 - 0.128e-6 * t12621 * t324 + 0.192e-6 * t12636 * t302 * t324 - 0.768e-9 * t12637 * t1146 + 0.32e-4 * t12547 * sigma2 * t362 - 0.128e-6 * t12548 * t1158 + 0.4e-2 * t12563 * t287 * t739 - 0.16e-4 * t12564 * t1164 + 0.4e-2 * t12589 * t287 * t1150 - 0.16e-4 * t12590 * t362;
        let t29611 = piecewise3(t235, 0.0, -3.0 / 8.0 * t6 * t244 * t29607);
        let tv4sigmatau311 = t7 * t29611;
        v4sigmatau3[ip * 12 + 11] += tv4sigmatau311;
        let tv4lapl40 = 0.0;
        v4lapl4[ip * 5] += tv4lapl40;
        let tv4lapl41 = 0.0;
        v4lapl4[ip * 5 + 1] += tv4lapl41;
        let tv4lapl42 = 0.0;
        v4lapl4[ip * 5 + 2] += tv4lapl42;
        let tv4lapl43 = 0.0;
        v4lapl4[ip * 5 + 3] += tv4lapl43;
        let tv4lapl44 = 0.0;
        v4lapl4[ip * 5 + 4] += tv4lapl44;
        let tv4lapl3tau0 = 0.0;
        v4lapl3tau[ip * 8] += tv4lapl3tau0;
        let tv4lapl3tau1 = 0.0;
        v4lapl3tau[ip * 8 + 1] += tv4lapl3tau1;
        let tv4lapl3tau2 = 0.0;
        v4lapl3tau[ip * 8 + 2] += tv4lapl3tau2;
        let tv4lapl3tau3 = 0.0;
        v4lapl3tau[ip * 8 + 3] += tv4lapl3tau3;
        let tv4lapl3tau4 = 0.0;
        v4lapl3tau[ip * 8 + 4] += tv4lapl3tau4;
        let tv4lapl3tau5 = 0.0;
        v4lapl3tau[ip * 8 + 5] += tv4lapl3tau5;
        let tv4lapl3tau6 = 0.0;
        v4lapl3tau[ip * 8 + 6] += tv4lapl3tau6;
        let tv4lapl3tau7 = 0.0;
        v4lapl3tau[ip * 8 + 7] += tv4lapl3tau7;
        let tv4lapl2tau20 = 0.0;
        v4lapl2tau2[ip * 9] += tv4lapl2tau20;
        let tv4lapl2tau21 = 0.0;
        v4lapl2tau2[ip * 9 + 1] += tv4lapl2tau21;
        let tv4lapl2tau22 = 0.0;
        v4lapl2tau2[ip * 9 + 2] += tv4lapl2tau22;
        let tv4lapl2tau23 = 0.0;
        v4lapl2tau2[ip * 9 + 3] += tv4lapl2tau23;
        let tv4lapl2tau24 = 0.0;
        v4lapl2tau2[ip * 9 + 4] += tv4lapl2tau24;
        let tv4lapl2tau25 = 0.0;
        v4lapl2tau2[ip * 9 + 5] += tv4lapl2tau25;
        let tv4lapl2tau26 = 0.0;
        v4lapl2tau2[ip * 9 + 6] += tv4lapl2tau26;
        let tv4lapl2tau27 = 0.0;
        v4lapl2tau2[ip * 9 + 7] += tv4lapl2tau27;
        let tv4lapl2tau28 = 0.0;
        v4lapl2tau2[ip * 9 + 8] += tv4lapl2tau28;
        let tv4lapltau30 = 0.0;
        v4lapltau3[ip * 8] += tv4lapltau30;
        let tv4lapltau31 = 0.0;
        v4lapltau3[ip * 8 + 1] += tv4lapltau31;
        let tv4lapltau32 = 0.0;
        v4lapltau3[ip * 8 + 2] += tv4lapltau32;
        let tv4lapltau33 = 0.0;
        v4lapltau3[ip * 8 + 3] += tv4lapltau33;
        let tv4lapltau34 = 0.0;
        v4lapltau3[ip * 8 + 4] += tv4lapltau34;
        let tv4lapltau35 = 0.0;
        v4lapltau3[ip * 8 + 5] += tv4lapltau35;
        let tv4lapltau36 = 0.0;
        v4lapltau3[ip * 8 + 6] += tv4lapltau36;
        let tv4lapltau37 = 0.0;
        v4lapltau3[ip * 8 + 7] += tv4lapltau37;
        let t30 = param_c_1;
        let t44 = t30 * t43;
        let t48 = param_c_2;
        let t50 = t48 * t49;
        let t54 = param_c_3;
        let t56 = t54 * t55;
        let t60 = param_c_4;
        let t62 = t60 * t61;
        let t66 = param_c_5;
        let t68 = t66 * t61 * t43;
        let t94 = t89 * t93;
        let t137 = param_c_19;
        let t138 = t137 * t43;
        let t140 = param_c_20;
        let t141 = t140 * t49;
        let t143 = param_c_21;
        let t144 = t143 * t55;
        let t146 = param_c_22;
        let t147 = t146 * t61;
        let t178 = t94 * t164;
        let t194 = param_c_31;
        let t195 = t194 * t43;
        let t197 = param_c_32;
        let t198 = t197 * t49;
        let t200 = param_c_33;
        let t201 = t200 * t55;
        let t216 = t94 * t205;
        let t220 = param_c_38;
        let t221 = t220 * t43;
        let t223 = param_c_39;
        let t224 = t223 * t49;
        let t227 = t204 * t163;
        let t228 = 1.0 / t227;
        let t456 = t110 * t87;
        let t615 = t54 * t49;
        let t620 = t60 * t55;
        let t625 = t66 * t61;
        let t636 = t48 * t43;
        let t646 = t197 * t43;
        let t651 = t200 * t49;
        let t663 = t223 * t43;
        let t675 = t140 * t43;
        let t680 = t143 * t49;
        let t685 = t146 * t55;
        let t1591 = t200 * t43;
        let t1628 = t143 * t43;
        let t1639 = t146 * t49;
        let t1846 = t54 * t43;
        let t1856 = t60 * t49;
        let t1863 = t66 * t55;
        let t5027 = t146 * t43;
        let t5167 = t63 * t63;
        let t5168 = 1.0 / t5167;
        let t5499 = t60 * t43;
        let t5564 = t66 * t49;
        let t12981 = 1.0 / t39 / t456;
        let t13540 = t66 * t43;
        let t13565 = 1.0 / t5167 / t45;
        let t29615 = t70 * t12981;
        let t29623 = t629 * t12981;
        let t29647 = t1831 * t12981;
        let t29682 = t5168 * t12981;
        let t29685 = 24.0 * t137 * t12981 * t64 + 72.0 * t140 * t12981 * t64 + 72.0 * t143 * t12981 * t64 + 24.0 * t146 * t12981 * t64 + 24.0 * t138 * t29615 + 120.0 * t141 * t29623 + 360.0 * t144 * t29647 + 840.0 * t147 * t29682 + 432.0 * t1628 * t29615 + 1440.0 * t1639 * t29623 + 384.0 * t5027 * t29615 + 192.0 * t675 * t29615 + 720.0 * t680 * t29623 + 1920.0 * t685 * t29647;
        let t29743 = 24.0 * t73 * t12981 * t64 + 72.0 * t76 * t12981 * t64 + 72.0 * t79 * t12981 * t64 + 24.0 * t82 * t12981 * t64 + 432.0 * t2002 * t29615 + 1440.0 * t2013 * t29623 + 192.0 * t473 * t29615 + 384.0 * t5190 * t29615 + 24.0 * t74 * t29615 + 720.0 * t480 * t29623 + 120.0 * t77 * t29623 + 1920.0 * t487 * t29647 + 360.0 * t80 * t29647 + 840.0 * t83 * t29682;
        let t29747 = (24.0 * t220 * t12981 * t64 + 72.0 * t223 * t12981 * t64 + 24.0 * t221 * t29615 + 120.0 * t224 * t29623 + 192.0 * t663 * t29615) * t228 + (24.0 * t194 * t12981 * t64 + 72.0 * t197 * t12981 * t64 + 72.0 * t200 * t12981 * t64 + 432.0 * t1591 * t29615 + 24.0 * t195 * t29615 + 120.0 * t198 * t29623 + 360.0 * t201 * t29647 + 192.0 * t646 * t29615 + 720.0 * t651 * t29623) * t205 + t29685 * t164 + 24.0 * t30 * t12981 * t64 + 72.0 * t48 * t12981 * t64 + 24.0 * t60 * t12981 * t64 + 72.0 * t54 * t12981 * t64 + 24.0 * t44 * t29615 + 120.0 * t50 * t29623 + 720.0 * t615 * t29623 + 360.0 * t56 * t29647 + 1920.0 * t620 * t29647 + 840.0 * t62 * t29682 + 0.4e-2 * t29743 * sigma0 * t94;
        let t29865 = 0.16e-4 * (72.0 * t101 * t12981 * t64 + 72.0 * t104 * t12981 * t64 + 24.0 * t98 * t12981 * t64 + 120.0 * t102 * t29623 + 360.0 * t105 * t29647 + 432.0 * t1904 * t29615 + 192.0 * t506 * t29615 + 24.0 * t99 * t29615 + 720.0 * t511 * t29623) * t108 * t116 + 0.64e-7 * (24.0 * t120 * t12981 * t64 + 72.0 * t123 * t12981 * t64 + 24.0 * t121 * t29615 + 120.0 * t124 * t29623 + 192.0 * t528 * t29615) * t127 * t133 + 120.0 * t13540 * t29615 + 4200.0 * t625 * t29682 + 1680.0 * t68 * t13565 * t12981 + 384.0 * t5499 * t29615 + 1200.0 * t5564 * t29623 + 192.0 * t636 * t29615 + 432.0 * t1846 * t29615 + 1440.0 * t1856 * t29623 + 3600.0 * t1863 * t29647 + 0.4e-2 * (24.0 * t208 * t12981 * t64 + 72.0 * t211 * t12981 * t64 + 24.0 * t209 * t29615 + 120.0 * t212 * t29623 + 192.0 * t580 * t29615) * sigma0 * t216 + 0.4e-2 * (24.0 * t167 * t12981 * t64 + 72.0 * t170 * t12981 * t64 + 72.0 * t173 * t12981 * t64 + 24.0 * t168 * t29615 + 120.0 * t171 * t29623 + 360.0 * t174 * t29647 + 432.0 * t1775 * t29615 + 192.0 * t598 * t29615 + 720.0 * t603 * t29623) * sigma0 * t178 + 0.16e-4 * (24.0 * t182 * t12981 * t64 + 72.0 * t185 * t12981 * t64 + 24.0 * t183 * t29615 + 120.0 * t186 * t29623 + 192.0 * t561 * t29615) * t108 * t190;
        let t29870 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t28 * (t29747 + t29865));
        let tv4tau40 = t7 * t29870;
        v4tau4[ip * 5] += tv4tau40;
        let tv4tau41 = 0.0;
        v4tau4[ip * 5 + 1] += tv4tau41;
        let tv4tau42 = 0.0;
        v4tau4[ip * 5 + 2] += tv4tau42;
        let tv4tau43 = 0.0;
        v4tau4[ip * 5 + 3] += tv4tau43;
        let t251 = t30 * t250;
        let t256 = t48 * t255;
        let t261 = t54 * t260;
        let t266 = t60 * t265;
        let t271 = t66 * t265 * t250;
        let t292 = t287 * t291;
        let t327 = t137 * t250;
        let t329 = t140 * t255;
        let t331 = t143 * t260;
        let t333 = t146 * t265;
        let t353 = t292 * t343;
        let t365 = t194 * t250;
        let t367 = t197 * t255;
        let t369 = t200 * t260;
        let t381 = t292 * t373;
        let t384 = t220 * t250;
        let t386 = t223 * t255;
        let t389 = t372 * t342;
        let t390 = 1.0 / t389;
        let t860 = t304 * t285;
        let t869 = t48 * t250;
        let t874 = t54 * t255;
        let t879 = t60 * t260;
        let t886 = t66 * t265;
        let t942 = t197 * t250;
        let t947 = t200 * t255;
        let t959 = t223 * t250;
        let t971 = t140 * t250;
        let t976 = t143 * t255;
        let t981 = t146 * t260;
        let t2598 = t200 * t250;
        let t2640 = t143 * t250;
        let t2651 = t146 * t255;
        let t2826 = t66 * t260;
        let t2846 = t54 * t250;
        let t2853 = t60 * t255;
        let t7378 = t66 * t255;
        let t7386 = t267 * t267;
        let t7387 = 1.0 / t7386;
        let t7512 = t60 * t250;
        let t7889 = t146 * t250;
        let t18196 = 1.0 / t7386 / t252;
        let t18274 = t66 * t250;
        let t18599 = 1.0 / t246 / t860;
        let t29874 = t273 * t18599;
        let t29882 = t890 * t18599;
        let t29892 = t2835 * t18599;
        let t29904 = t7387 * t18599;
        let t29907 = 24.0 * t137 * t18599 * t268 + 72.0 * t140 * t18599 * t268 + 72.0 * t143 * t18599 * t268 + 24.0 * t146 * t18599 * t268 + 432.0 * t2640 * t29874 + 1440.0 * t2651 * t29882 + 24.0 * t327 * t29874 + 384.0 * t7889 * t29874 + 192.0 * t971 * t29874 + 120.0 * t329 * t29882 + 720.0 * t976 * t29882 + 360.0 * t331 * t29892 + 1920.0 * t981 * t29892 + 840.0 * t333 * t29904;
        let t29990 = 24.0 * t73 * t18599 * t268 + 72.0 * t76 * t18599 * t268 + 72.0 * t79 * t18599 * t268 + 24.0 * t82 * t18599 * t268 + 192.0 * t1005 * t29874 + 720.0 * t1010 * t29882 + 1920.0 * t1015 * t29892 + 24.0 * t275 * t29874 + 120.0 * t277 * t29882 + 360.0 * t279 * t29892 + 840.0 * t281 * t29904 + 432.0 * t2876 * t29874 + 1440.0 * t2887 * t29882 + 384.0 * t7284 * t29874;
        let t30043 = t29907 * t343 + (24.0 * t194 * t18599 * t268 + 72.0 * t197 * t18599 * t268 + 72.0 * t200 * t18599 * t268 + 432.0 * t2598 * t29874 + 24.0 * t365 * t29874 + 192.0 * t942 * t29874 + 120.0 * t367 * t29882 + 720.0 * t947 * t29882 + 360.0 * t369 * t29892) * t373 + (24.0 * t220 * t18599 * t268 + 72.0 * t223 * t18599 * t268 + 24.0 * t384 * t29874 + 192.0 * t959 * t29874 + 120.0 * t386 * t29882) * t390 + 24.0 * t60 * t18599 * t268 + 72.0 * t48 * t18599 * t268 + 72.0 * t54 * t18599 * t268 + 24.0 * t30 * t18599 * t268 + 0.4e-2 * t29990 * sigma2 * t292 + 1920.0 * t879 * t29892 + 3600.0 * t2826 * t29892 + 432.0 * t2846 * t29874 + 1440.0 * t2853 * t29882 + 0.16e-4 * (72.0 * t101 * t18599 * t268 + 72.0 * t104 * t18599 * t268 + 24.0 * t98 * t18599 * t268 + 192.0 * t1032 * t29874 + 720.0 * t1037 * t29882 + 432.0 * t2929 * t29874 + 24.0 * t295 * t29874 + 120.0 * t297 * t29882 + 360.0 * t299 * t29892) * t302 * t310 + 0.64e-7 * (24.0 * t120 * t18599 * t268 + 72.0 * t123 * t18599 * t268 + 192.0 * t1054 * t29874 + 24.0 * t313 * t29874 + 120.0 * t315 * t29882) * t318 * t324;
        let t30124 = 120.0 * t18274 * t29874 + 24.0 * t251 * t29874 + 192.0 * t869 * t29874 + 120.0 * t256 * t29882 + 720.0 * t874 * t29882 + 360.0 * t261 * t29892 + 840.0 * t266 * t29904 + 4200.0 * t886 * t29904 + 384.0 * t7512 * t29874 + 1200.0 * t7378 * t29882 + 1680.0 * t271 * t18196 * t18599 + 0.4e-2 * (24.0 * t167 * t18599 * t268 + 72.0 * t170 * t18599 * t268 + 72.0 * t173 * t18599 * t268 + 432.0 * t2811 * t29874 + 24.0 * t345 * t29874 + 192.0 * t820 * t29874 + 120.0 * t347 * t29882 + 720.0 * t827 * t29882 + 360.0 * t349 * t29892) * sigma2 * t353 + 0.16e-4 * (24.0 * t182 * t18599 * t268 + 72.0 * t185 * t18599 * t268 + 24.0 * t356 * t29874 + 192.0 * t843 * t29874 + 120.0 * t358 * t29882) * t302 * t362 + 0.4e-2 * (24.0 * t208 * t18599 * t268 + 72.0 * t211 * t18599 * t268 + 24.0 * t375 * t29874 + 192.0 * t916 * t29874 + 120.0 * t377 * t29882) * sigma2 * t381;
        let t30129 = piecewise3(t235, 0.0, -3.0 / 8.0 * t6 * t244 * (t30043 + t30124));
        let tv4tau44 = t7 * t30129;
        v4tau4[ip * 5 + 4] += tv4tau44;
    }
}
