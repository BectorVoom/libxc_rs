//! MGGA_X_MBRXC_BG exc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_mbrxc_bg.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::mbrxc::{xc_mgga_x_mbrxc_get_x};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_x_mbrxc_bg_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
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
        let t19 = piecewise5::<f64>(t11, t12, t15, t16, t17 * t8);
        let t20 = 1.0 + t19;
        let t21 = t20 <= zeta_threshold;
        let t22 = pow_1_3::<f64>(zeta_threshold);
        let t23 = t22 * zeta_threshold;
        let t24 = pow_1_3::<f64>(t20);
        let t26 = piecewise3::<f64>(t21, t23, t24 * t20);
        let t27 = pow_1_3::<f64>(t7);
        let t28 = t26 * t27;
        let t29 = pow_1_3::<f64>(rho0);
        let t30 = t29 * t29;
        let t32 = 1.0 / t30 / rho0;
        let t35 = M_CBRT6;
        let t36 = t35 * t35;
        let t37 = M_PI * M_PI;
        let t38 = pow_1_3::<f64>(t37);
        let t39 = t38 * t38;
        let t41 = 3.0 / 10.0 * t36 * t39;
        let t42 = rho0 * rho0;
        let t44 = 1.0 / t30 / t42;
        let t47 = sigma0 * sigma0;
        let t48 = t42 * t42;
        let t49 = t48 * rho0;
        let t51 = 1.0 / t29 / t49;
        let t54 = 0.149492e0 * tau0 * t32 - t41 + 0.147e0 * sigma0 * t44 + 0.32e-2 * t47 * t51;
        let t55 = xc_mgga_x_mbrxc_get_x::<f64>(t54);
        let t56 = pow_1_4::<f64>(f64::EPSILON);
        let t57 = t55 < t56;
        let t58 = pow_1_3::<f64>(32.0);
        let t59 = t58 * t4;
        let t60 = t3 * t3;
        let t62 = pow_1_3::<f64>(1.0 / M_PI);
        let t63 = 1.0 / t62;
        let t64 = t60 * t63;
        let t65 = M_CBRT4;
        let t66 = t64 * t65;
        let t67 = t59 * t66;
        let t68 = t67 / 12.0;
        let t69 = t59 * t60;
        let t70 = t63 * t65;
        let t71 = t55 * t55;
        let t72 = t70 * t71;
        let t75 = t71 * t55;
        let t76 = t70 * t75;
        let t79 = t71 * t71;
        let t80 = t70 * t79;
        let t83 = t79 * t55;
        let t84 = t70 * t83;
        let t87 = t79 * t71;
        let t88 = t70 * t87;
        let t96 = t59 * t64;
        let t97 = t56 < t55;
        let t98 = piecewise3::<f64>(t97, t55, t56);
        let t100 = f64::exp(t98 / 3.0);
        let t101 = t65 * t100;
        let t102 = f64::exp(-t98);
        let t103 = t98 * t98;
        let t105 = t103 + 5.0 * t98 + 8.0;
        let t106 = t102 * t105;
        let t107 = 8.0 - t106;
        let t108 = 1.0 / t98;
        let t109 = t107 * t108;
        let t110 = 1.0 + t98;
        let t111 = pow_1_3::<f64>(t110);
        let t112 = 1.0 / t111;
        let t113 = t109 * t112;
        let t117 = piecewise3::<f64>(t57, -t68 - t69 * t72 / 108.0 + t69 * t76 / 108.0 - 13.0 / 1620.0 * t69 * t80 + 67.0 / 9720.0 * t69 * t84 - 52.0 / 8505.0 * t69 * t88 + 1811.0 / 326592.0 * t69 * t70 * t79 * t75, -t96 * t101 * t113 / 36.0);
        let t121 = piecewise3::<f64>(t2, 0.0, 3.0 / 16.0 * t6 * t28 * t117);
        let t122 = rho1 <= dens_threshold;
        let t123 = -t17;
        let t125 = piecewise5::<f64>(t15, t12, t11, t16, t123 * t8);
        let t126 = 1.0 + t125;
        let t127 = t126 <= zeta_threshold;
        let t128 = pow_1_3::<f64>(t126);
        let t130 = piecewise3::<f64>(t127, t23, t128 * t126);
        let t131 = t130 * t27;
        let t132 = pow_1_3::<f64>(rho1);
        let t133 = t132 * t132;
        let t135 = 1.0 / t133 / rho1;
        let t138 = rho1 * rho1;
        let t140 = 1.0 / t133 / t138;
        let t143 = sigma2 * sigma2;
        let t144 = t138 * t138;
        let t145 = t144 * rho1;
        let t147 = 1.0 / t132 / t145;
        let t150 = 0.149492e0 * tau1 * t135 - t41 + 0.147e0 * sigma2 * t140 + 0.32e-2 * t143 * t147;
        let t151 = xc_mgga_x_mbrxc_get_x::<f64>(t150);
        let t152 = t151 < t56;
        let t153 = t151 * t151;
        let t154 = t70 * t153;
        let t157 = t153 * t151;
        let t158 = t70 * t157;
        let t161 = t153 * t153;
        let t162 = t70 * t161;
        let t165 = t161 * t151;
        let t166 = t70 * t165;
        let t169 = t161 * t153;
        let t170 = t70 * t169;
        let t178 = t56 < t151;
        let t179 = piecewise3::<f64>(t178, t151, t56);
        let t181 = f64::exp(t179 / 3.0);
        let t182 = t65 * t181;
        let t183 = f64::exp(-t179);
        let t184 = t179 * t179;
        let t186 = t184 + 5.0 * t179 + 8.0;
        let t187 = t183 * t186;
        let t188 = 8.0 - t187;
        let t189 = 1.0 / t179;
        let t190 = t188 * t189;
        let t191 = 1.0 + t179;
        let t192 = pow_1_3::<f64>(t191);
        let t193 = 1.0 / t192;
        let t194 = t190 * t193;
        let t198 = piecewise3::<f64>(t152, -t68 - t69 * t154 / 108.0 + t69 * t158 / 108.0 - 13.0 / 1620.0 * t69 * t162 + 67.0 / 9720.0 * t69 * t166 - 52.0 / 8505.0 * t69 * t170 + 1811.0 / 326592.0 * t69 * t70 * t161 * t157, -t96 * t182 * t194 / 36.0);
        let t202 = piecewise3::<f64>(t122, 0.0, 3.0 / 16.0 * t6 * t131 * t198);
        let tzk0 = t121 + t202;
        zk[ip] += tzk0;
    }
}
