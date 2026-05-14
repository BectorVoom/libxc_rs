//! MGGA_C_B88 vxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_b88.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_c_b88_vxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    vlapl: &mut Array<f64>,
    vtau: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = t4 * t4;
        let t6 = 1.0 / M_PI;
        let t7 = pow_1_3(t6);
        let t8 = 1.0 / t7;
        let t9 = t5 * t8;
        let t10 = M_CBRT4;
        let t11 = t9 * t10;
        let t12 = M_CBRT2;
        let t13 = 1.0 <= zeta_threshold;
        let t14 = zeta_threshold - 1.0;
        let t16 = piecewise5(t13, t14, t13, -t14, 0.0);
        let t17 = 1.0 + t16;
        let t18 = t17 * rho[ip];
        let t19 = pow_1_3(t18);
        let t20 = 1.0 / t19;
        let t21 = t12 * t20;
        let t22 = t12 * t12;
        let t23 = sigma[ip] * t22;
        let t24 = rho[ip] * rho[ip];
        let t25 = pow_1_3(rho[ip]);
        let t26 = t25 * t25;
        let t28 = 1.0 / t26 / t24;
        let t29 = t23 * t28;
        let t31 = 1.0 + 0.7e-2 * t29;
        let t32 = f64::powf(t31, 1.0 / 5.0);
        let t33 = t32 * t32;
        let t34 = t33 * t33;
        let t35 = 1.0 / t34;
        let t40 = 1.0 + 0.83333333333333333333e-3 * t11 * t23 * t28 * t35;
        let t41 = 1.0 / t40;
        let t43 = t11 * t21 * t41;
        let t45 = piecewise3(t3, 0.0, t43 / 9.0);
        let t46 = rho[ip] * t45;
        let t47 = 0.126e1 * t45;
        let t48 = 1.0 + t47;
        let t49 = f64::ln(t48);
        let t50 = t47 - t49;
        let t52 = 0.252e0 * t46 * t50;
        let t53 = t17 * t17;
        let t54 = pow_1_3(t17);
        let t55 = t54 * t54;
        let t56 = t55 * t53;
        let t57 = t56 * t22;
        let t58 = t26 * rho[ip];
        let t59 = tau[ip] * t22;
        let t64 = 2.0 * t59 / t58 - t29 / 4.0;
        let t66 = t58 * t64 * t5;
        let t67 = t57 * t66;
        let t69 = 1.0 / t7 / t6;
        let t70 = t69 * t10;
        let t72 = 1.0 / t19 / t18;
        let t73 = t40 * t40;
        let t74 = t73 * t73;
        let t75 = 1.0 / t74;
        let t76 = t72 * t75;
        let t78 = 1.0 + 0.10666666666666666667e0 * t43;
        let t79 = f64::ln(t78);
        let t80 = t79 * t4;
        let t81 = t80 * t7;
        let t82 = t10 * t10;
        let t83 = t82 * t22;
        let t84 = t19 * t40;
        let t85 = t83 * t84;
        let t88 = 1.0 - 0.390625e0 * t81 * t85;
        let t90 = t70 * t76 * t88;
        let t93 = piecewise3(t3, 0.0, -0.18641351111111111112e-3 * t67 * t90);
        let t94 = 2.0 * t93;
        let tzk0 = -t52 + t94;
        zk[ip] += tzk0;
        let t95 = t45 * t50;
        let t97 = t12 * t72;
        let t100 = t11 * t97 * t41 * t17;
        let t102 = 1.0 / t73;
        let t103 = t24 * rho[ip];
        let t105 = 1.0 / t26 / t103;
        let t110 = sigma[ip] * sigma[ip];
        let t111 = t110 * t12;
        let t112 = t24 * t24;
        let t113 = t112 * t24;
        let t115 = 1.0 / t25 / t113;
        let t117 = 1.0 / t34 / t31;
        let t122 = -0.22222222222222222222e-2 * t11 * t23 * t105 * t35 + 0.24888888888888888889e-4 * t11 * t111 * t115 * t117;
        let t125 = t11 * t21 * t102 * t122;
        let t128 = piecewise3(t3, 0.0, -t100 / 27.0 - t125 / 9.0);
        let t129 = rho[ip] * t128;
        let t130 = t129 * t50;
        let t133 = 1.0 / t48;
        let t136 = 0.126e1 * t128 - 0.126e1 * t128 * t133;
        let t137 = t46 * t136;
        let t140 = t26 * t64 * t5;
        let t141 = t57 * t140;
        let t148 = -10.0 / 3.0 * t59 * t28 + 2.0 / 3.0 * t23 * t105;
        let t150 = t58 * t148 * t5;
        let t151 = t57 * t150;
        let t154 = t53 * t17;
        let t155 = t55 * t154;
        let t156 = t155 * t22;
        let t157 = t156 * t66;
        let t158 = t53 * t24;
        let t160 = 1.0 / t19 / t158;
        let t161 = t160 * t75;
        let t163 = t70 * t161 * t88;
        let t166 = t70 * t72;
        let t168 = 1.0 / t74 / t40;
        let t169 = t168 * t88;
        let t170 = t169 * t122;
        let t171 = t166 * t170;
        let t176 = -0.35555555555555555557e-1 * t100 - 0.10666666666666666667e0 * t125;
        let t177 = 1.0 / t78;
        let t179 = t4 * t7;
        let t180 = t176 * t177 * t179;
        let t184 = t80 * t7 * t82;
        let t185 = t19 * t19;
        let t186 = 1.0 / t185;
        let t187 = t22 * t186;
        let t188 = t40 * t17;
        let t192 = t19 * t122;
        let t193 = t83 * t192;
        let t196 = -0.390625e0 * t180 * t85 - 0.13020833333333333333e0 * t184 * t187 * t188 - 0.390625e0 * t81 * t193;
        let t198 = t70 * t76 * t196;
        let t202 = piecewise3(t3, 0.0, -0.3106891851851851852e-3 * t141 * t90 - 0.18641351111111111112e-3 * t151 * t90 + 0.24855134814814814816e-3 * t157 * t163 + 0.74565404444444444448e-3 * t67 * t171 - 0.18641351111111111112e-3 * t67 * t198);
        let tvrho0 = -t52 + t94 + rho[ip] * (-0.252e0 * t95 - 0.252e0 * t130 - 0.252e0 * t137 + 2.0 * t202);
        vrho[ip] += tvrho0;
        let t211 = t112 * rho[ip];
        let t213 = 1.0 / t25 / t211;
        let t218 = 0.83333333333333333333e-3 * t11 * t22 * t28 * t35 - 0.93333333333333333333e-5 * t11 * sigma[ip] * t12 * t213 * t117;
        let t219 = t102 * t218;
        let t223 = piecewise3(t3, 0.0, -t11 * t21 * t219 / 9.0);
        let t224 = rho[ip] * t223;
        let t226 = 0.252e0 * t224 * t50;
        let t230 = 0.126e1 * t223 - 0.126e1 * t223 * t133;
        let t232 = 0.252e0 * t46 * t230;
        let t233 = t56 * t12;
        let t234 = 1.0 / rho[ip];
        let t235 = t234 * t5;
        let t236 = t233 * t235;
        let t239 = t169 * t218;
        let t240 = t166 * t239;
        let t243 = t41 * t218;
        let t246 = t19 * t218;
        let t247 = t83 * t246;
        let t250 = 1.0 * t243 * t177 - 0.390625e0 * t81 * t247;
        let t252 = t70 * t76 * t250;
        let t256 = piecewise3(t3, 0.0, 0.9320675555555555556e-4 * t236 * t90 + 0.74565404444444444448e-3 * t67 * t240 - 0.18641351111111111112e-3 * t67 * t252);
        let t257 = 2.0 * t256;
        let tvsigma0 = rho[ip] * (-t226 - t232 + t257);
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t259 = t5 * t69;
        let t260 = t233 * t259;
        let t261 = t10 * t72;
        let t262 = t75 * t88;
        let t266 = piecewise3(t3, 0.0, -0.74565404444444444448e-3 * t260 * t261 * t262);
        let tvtau0 = 2.0 * rho[ip] * t266;
        vtau[ip] += tvtau0;
    }
}
