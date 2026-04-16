//! GGA_K_APBEINT vxc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_apbeint.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_k_apbeint_vxc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    param_alpha: f64,
    param_kappa: f64,
    param_muGE: f64,
    param_muPBE: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let t1 = rho0 <= dens_threshold;
        let t2 = M_CBRT3;
        let t3 = t2 * t2;
        let t4 = M_CBRTPI;
        let t6 = t3 * t4 * M_PI;
        let t7 = rho0 + rho1;
        let t8 = 1.0 / t7;
        let t11 = 2.0 * rho0 * t8 <= zeta_threshold;
        let t12 = zeta_threshold - 1.0;
        let t15 = 2.0 * rho1 * t8 <= zeta_threshold;
        let t16 = -t12;
        let t17 = rho0 - rho1;
        let t19 = piecewise5(t11, t12, t15, t16, t17 * t8);
        let t20 = 1.0 + t19;
        let t21 = t20 <= zeta_threshold;
        let t22 = pow_1_3(zeta_threshold);
        let t23 = t22 * t22;
        let t24 = t23 * zeta_threshold;
        let t25 = pow_1_3(t20);
        let t26 = t25 * t25;
        let t28 = piecewise3(t21, t24, t26 * t20);
        let t29 = pow_1_3(t7);
        let t30 = t29 * t29;
        let t31 = t28 * t30;
        let t32 = param_muPBE - param_muGE;
        let t34 = M_CBRT6;
        let t35 = t32 * param_alpha * t34;
        let t36 = M_PI * M_PI;
        let t37 = pow_1_3(t36);
        let t38 = t37 * t37;
        let t39 = 1.0 / t38;
        let t40 = t39 * sigma0;
        let t41 = rho0 * rho0;
        let t42 = pow_1_3(rho0);
        let t43 = t42 * t42;
        let t45 = 1.0 / t43 / t41;
        let t46 = param_alpha * t34;
        let t47 = t40 * t45;
        let t50 = 1.0 + t46 * t47 / 24.0;
        let t51 = 1.0 / t50;
        let t57 = (param_muGE + t35 * t40 * t45 * t51 / 24.0) * t34;
        let t60 = param_kappa + t57 * t47 / 24.0;
        let t65 = 1.0 + param_kappa * (1.0 - param_kappa / t60);
        let t69 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t31 * t65);
        let t70 = rho1 <= dens_threshold;
        let t71 = -t17;
        let t73 = piecewise5(t15, t12, t11, t16, t71 * t8);
        let t74 = 1.0 + t73;
        let t75 = t74 <= zeta_threshold;
        let t76 = pow_1_3(t74);
        let t77 = t76 * t76;
        let t79 = piecewise3(t75, t24, t77 * t74);
        let t80 = t79 * t30;
        let t81 = t39 * sigma2;
        let t82 = rho1 * rho1;
        let t83 = pow_1_3(rho1);
        let t84 = t83 * t83;
        let t86 = 1.0 / t84 / t82;
        let t87 = t81 * t86;
        let t90 = 1.0 + t46 * t87 / 24.0;
        let t91 = 1.0 / t90;
        let t97 = (param_muGE + t35 * t81 * t86 * t91 / 24.0) * t34;
        let t100 = param_kappa + t97 * t87 / 24.0;
        let t105 = 1.0 + param_kappa * (1.0 - param_kappa / t100);
        let t109 = piecewise3(t70, 0.0, 3.0 / 20.0 * t6 * t80 * t105);
        let tzk0 = t69 + t109;
        zk[ip] += tzk0;
        let t110 = t7 * t7;
        let t111 = 1.0 / t110;
        let t112 = t17 * t111;
        let t114 = piecewise5(t11, 0.0, t15, 0.0, t8 - t112);
        let t117 = piecewise3(t21, 0.0, 5.0 / 3.0 * t26 * t114);
        let t118 = t117 * t30;
        let t122 = 1.0 / t29;
        let t123 = t28 * t122;
        let t126 = t6 * t123 * t65 / 10.0;
        let t127 = t6 * t28;
        let t128 = param_kappa * param_kappa;
        let t129 = t30 * t128;
        let t130 = t60 * t60;
        let t131 = 1.0 / t130;
        let t132 = t41 * rho0;
        let t134 = 1.0 / t43 / t132;
        let t139 = param_alpha * param_alpha;
        let t141 = t34 * t34;
        let t142 = t32 * t139 * t141;
        let t144 = 1.0 / t37 / t36;
        let t145 = sigma0 * sigma0;
        let t146 = t144 * t145;
        let t147 = t41 * t41;
        let t148 = t147 * t41;
        let t150 = 1.0 / t42 / t148;
        let t151 = t50 * t50;
        let t152 = 1.0 / t151;
        let t158 = (-t35 * t40 * t134 * t51 / 9.0 + t142 * t146 * t150 * t152 / 216.0) * t34;
        let t161 = t40 * t134;
        let t164 = t158 * t47 / 24.0 - t57 * t161 / 9.0;
        let t165 = t131 * t164;
        let t166 = t129 * t165;
        let t170 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t118 * t65 + t126 + 3.0 / 20.0 * t127 * t166);
        let t171 = t71 * t111;
        let t173 = piecewise5(t15, 0.0, t11, 0.0, -t8 - t171);
        let t176 = piecewise3(t75, 0.0, 5.0 / 3.0 * t77 * t173);
        let t177 = t176 * t30;
        let t181 = t79 * t122;
        let t184 = t6 * t181 * t105 / 10.0;
        let t186 = piecewise3(t70, 0.0, 3.0 / 20.0 * t6 * t177 * t105 + t184);
        let tvrho0 = t69 + t109 + t7 * (t170 + t186);
        vrho[ip * 2] += tvrho0;
        let t190 = piecewise5(t11, 0.0, t15, 0.0, -t8 - t112);
        let t193 = piecewise3(t21, 0.0, 5.0 / 3.0 * t26 * t190);
        let t194 = t193 * t30;
        let t199 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t194 * t65 + t126);
        let t201 = piecewise5(t15, 0.0, t11, 0.0, t8 - t171);
        let t204 = piecewise3(t75, 0.0, 5.0 / 3.0 * t77 * t201);
        let t205 = t204 * t30;
        let t209 = t6 * t79;
        let t210 = t100 * t100;
        let t211 = 1.0 / t210;
        let t212 = t82 * rho1;
        let t214 = 1.0 / t84 / t212;
        let t219 = sigma2 * sigma2;
        let t220 = t144 * t219;
        let t221 = t82 * t82;
        let t222 = t221 * t82;
        let t224 = 1.0 / t83 / t222;
        let t225 = t90 * t90;
        let t226 = 1.0 / t225;
        let t232 = (-t35 * t81 * t214 * t91 / 9.0 + t142 * t220 * t224 * t226 / 216.0) * t34;
        let t235 = t81 * t214;
        let t238 = t232 * t87 / 24.0 - t97 * t235 / 9.0;
        let t239 = t211 * t238;
        let t240 = t129 * t239;
        let t244 = piecewise3(t70, 0.0, 3.0 / 20.0 * t6 * t205 * t105 + t184 + 3.0 / 20.0 * t209 * t240);
        let tvrho1 = t69 + t109 + t7 * (t199 + t244);
        vrho[ip * 2 + 1] += tvrho1;
        let t247 = t39 * t45;
        let t252 = t147 * rho0;
        let t254 = 1.0 / t42 / t252;
        let t260 = (t35 * t247 * t51 / 24.0 - t142 * t144 * sigma0 * t254 * t152 / 576.0) * t34;
        let t264 = t57 * t247 / 24.0 + t260 * t47 / 24.0;
        let t265 = t131 * t264;
        let t266 = t129 * t265;
        let t269 = piecewise3(t1, 0.0, 3.0 / 20.0 * t127 * t266);
        let tvsigma0 = t7 * t269;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t270 = t39 * t86;
        let t275 = t221 * rho1;
        let t277 = 1.0 / t83 / t275;
        let t283 = (t35 * t270 * t91 / 24.0 - t142 * t144 * sigma2 * t277 * t226 / 576.0) * t34;
        let t287 = t97 * t270 / 24.0 + t283 * t87 / 24.0;
        let t288 = t211 * t287;
        let t289 = t129 * t288;
        let t292 = piecewise3(t70, 0.0, 3.0 / 20.0 * t209 * t289);
        let tvsigma2 = t7 * t292;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
