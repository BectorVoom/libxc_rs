//! GGA_K_PW86 vxc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 69 shared lines across all orders.
//! Delta: 60 lines unique to vxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn gga_k_pw86_vxc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
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
        // --- shared preamble (69 lines) ---
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
        let t32 = M_CBRT6;
        let t33 = M_PI * M_PI;
        let t34 = pow_1_3(t33);
        let t35 = t34 * t34;
        let t37 = t32 / t35;
        let t38 = rho0 * rho0;
        let t39 = pow_1_3(rho0);
        let t40 = t39 * t39;
        let t42 = 1.0 / t40 / t38;
        let t46 = t32 * t32;
        let t49 = t46 / t34 / t33;
        let t50 = sigma0 * sigma0;
        let t51 = t38 * t38;
        let t52 = t51 * rho0;
        let t54 = 1.0 / t39 / t52;
        let t58 = t50 * sigma0;
        let t59 = t51 * t51;
        let t60 = 1.0 / t59;
        let t63 = 1.0 + 0.91999999999999999998e-1 * t37 * sigma0 * t42 + 0.1609375e-1 * t49 * t50 * t54 + 0.89114429294134854068e-6 * t58 * t60;
        let t64 = f64::powf(t63, 1.0 / 15.0);
        let t68 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t31 * t64);
        let t69 = rho1 <= dens_threshold;
        let t70 = -t17;
        let t72 = piecewise5(t15, t12, t11, t16, t70 * t8);
        let t73 = 1.0 + t72;
        let t74 = t73 <= zeta_threshold;
        let t75 = pow_1_3(t73);
        let t76 = t75 * t75;
        let t78 = piecewise3(t74, t24, t76 * t73);
        let t79 = t78 * t30;
        let t80 = rho1 * rho1;
        let t81 = pow_1_3(rho1);
        let t82 = t81 * t81;
        let t84 = 1.0 / t82 / t80;
        let t88 = sigma2 * sigma2;
        let t89 = t80 * t80;
        let t90 = t89 * rho1;
        let t92 = 1.0 / t81 / t90;
        let t96 = t88 * sigma2;
        let t97 = t89 * t89;
        let t98 = 1.0 / t97;
        let t101 = 1.0 + 0.91999999999999999998e-1 * t37 * sigma2 * t84 + 0.1609375e-1 * t49 * t88 * t92 + 0.89114429294134854068e-6 * t96 * t98;
        let t102 = f64::powf(t101, 1.0 / 15.0);
        let t106 = piecewise3(t69, 0.0, 3.0 / 20.0 * t6 * t79 * t102);
        let tzk0 = t68 + t106;
        zk[ip] += tzk0;
        // --- vxc delta (this level) (60 lines) ---
        let t107 = t7 * t7;
        let t108 = 1.0 / t107;
        let t109 = t17 * t108;
        let t111 = piecewise5(t11, 0.0, t15, 0.0, t8 - t109);
        let t114 = piecewise3(t21, 0.0, 5.0 / 3.0 * t26 * t111);
        let t119 = 1.0 / t29;
        let t123 = t6 * t28 * t119 * t64 / 10.0;
        let t124 = t6 * t28;
        let t125 = t64 * t64;
        let t126 = t125 * t125;
        let t128 = t126 * t126;
        let t129 = t128 * t126 * t125;
        let t130 = 1.0 / t129;
        let t131 = t30 * t130;
        let t132 = t38 * rho0;
        let t134 = 1.0 / t40 / t132;
        let t138 = t51 * t38;
        let t140 = 1.0 / t39 / t138;
        let t144 = t59 * rho0;
        let t145 = 1.0 / t144;
        let t148 = -0.24533333333333333333e0 * t37 * sigma0 * t134 - 0.85833333333333333333e-1 * t49 * t50 * t140 - 0.71291543435307883254e-5 * t58 * t145;
        let t149 = t131 * t148;
        let t153 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t114 * t30 * t64 + t123 + t124 * t149 / 100.0);
        let t154 = t70 * t108;
        let t156 = piecewise5(t15, 0.0, t11, 0.0, -t8 - t154);
        let t159 = piecewise3(t74, 0.0, 5.0 / 3.0 * t76 * t156);
        let t167 = t6 * t78 * t119 * t102 / 10.0;
        let t169 = piecewise3(t69, 0.0, 3.0 / 20.0 * t6 * t159 * t30 * t102 + t167);
        let tvrho0 = t68 + t106 + t7 * (t153 + t169);
        vrho[ip * 2] += tvrho0;
        let t173 = piecewise5(t11, 0.0, t15, 0.0, -t8 - t109);
        let t176 = piecewise3(t21, 0.0, 5.0 / 3.0 * t26 * t173);
        let t182 = piecewise3(t1, 0.0, 3.0 / 20.0 * t6 * t176 * t30 * t64 + t123);
        let t184 = piecewise5(t15, 0.0, t11, 0.0, t8 - t154);
        let t187 = piecewise3(t74, 0.0, 5.0 / 3.0 * t76 * t184);
        let t192 = t6 * t78;
        let t193 = t102 * t102;
        let t194 = t193 * t193;
        let t196 = t194 * t194;
        let t197 = t196 * t194 * t193;
        let t198 = 1.0 / t197;
        let t199 = t30 * t198;
        let t200 = t80 * rho1;
        let t202 = 1.0 / t82 / t200;
        let t206 = t89 * t80;
        let t208 = 1.0 / t81 / t206;
        let t212 = t97 * rho1;
        let t213 = 1.0 / t212;
        let t216 = -0.24533333333333333333e0 * t37 * sigma2 * t202 - 0.85833333333333333333e-1 * t49 * t88 * t208 - 0.71291543435307883254e-5 * t96 * t213;
        let t217 = t199 * t216;
        let t221 = piecewise3(t69, 0.0, 3.0 / 20.0 * t6 * t187 * t30 * t102 + t167 + t192 * t217 / 100.0);
        let tvrho1 = t68 + t106 + t7 * (t182 + t221);
        vrho[ip * 2 + 1] += tvrho1;
        let t231 = 0.91999999999999999998e-1 * t37 * t42 + 0.321875e-1 * t49 * sigma0 * t54 + 0.2673432878824045622e-5 * t50 * t60;
        let t232 = t131 * t231;
        let t235 = piecewise3(t1, 0.0, t124 * t232 / 100.0);
        let tvsigma0 = t7 * t235;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t243 = 0.91999999999999999998e-1 * t37 * t84 + 0.321875e-1 * t49 * sigma2 * t92 + 0.2673432878824045622e-5 * t88 * t98;
        let t244 = t199 * t243;
        let t247 = piecewise3(t69, 0.0, t192 * t244 / 100.0);
        let tvsigma2 = t7 * t247;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
