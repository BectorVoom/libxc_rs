//! MGGA_X_PBE_GX exc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_pbe_gx.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{Heaviside, piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_x_pbe_gx_exc_pol(
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
        let t27 = t6 * t26;
        let t28 = pow_1_3::<f64>(t7);
        let t29 = M_CBRT2;
        let t30 = t3 * t3;
        let t32 = M_CBRT4;
        let t34 = 8.0 / 27.0 * t29 * t30 * t32;
        let t35 = pow_1_3::<f64>(rho0);
        let t36 = t35 * t35;
        let t38 = 1.0 / t36 / rho0;
        let t40 = rho0 * rho0;
        let t42 = 1.0 / t36 / t40;
        let t43 = sigma0 * t42;
        let t45 = tau0 * t38 - t43 / 8.0;
        let t46 = M_CBRT6;
        let t48 = M_PI * M_PI;
        let t49 = pow_1_3::<f64>(t48);
        let t50 = t49 * t49;
        let t51 = 1.0 / t50;
        let t52 = t45 * t46 * t51;
        let t54 = 0.827411e0 - 0.35753333333333333333e0 * t52;
        let t56 = 1.0 - 0.45341611111111111111e0 * t52;
        let t57 = 1.0 / t56;
        let t59 = 1.0 - t34;
        let t60 = t54 * t57 * t59;
        let t63 = t34 + 5.0 / 9.0 * t52 * t60;
        let t64 = 5.0 / 9.0 * t52;
        let t65 = 1.0 - t64;
        let t66 = Heaviside::<f64>(t65);
        let t68 = 1.0 + t64;
        let t69 = 1.0 / t68;
        let t72 = 1.0 + 0.148e0 * t65 * t69;
        let t73 = -t65;
        let t74 = Heaviside::<f64>(t73);
        let t76 = t63 * t66 + t72 * t74;
        let t79 = 1.0 + 0.1015549e-2 * t43;
        let t80 = 1.0 / t79;
        let t81 = t28 * t76 * t80;
        let t84 = piecewise3::<f64>(t2, 0.0, -3.0 / 8.0 * t27 * t81);
        let t85 = rho1 <= dens_threshold;
        let t86 = -t17;
        let t88 = piecewise5::<f64>(t15, t12, t11, t16, t86 * t8);
        let t89 = 1.0 + t88;
        let t90 = t89 <= zeta_threshold;
        let t91 = pow_1_3::<f64>(t89);
        let t93 = piecewise3::<f64>(t90, t23, t91 * t89);
        let t94 = t6 * t93;
        let t95 = pow_1_3::<f64>(rho1);
        let t96 = t95 * t95;
        let t98 = 1.0 / t96 / rho1;
        let t100 = rho1 * rho1;
        let t102 = 1.0 / t96 / t100;
        let t103 = sigma2 * t102;
        let t105 = tau1 * t98 - t103 / 8.0;
        let t107 = t105 * t46 * t51;
        let t109 = 0.827411e0 - 0.35753333333333333333e0 * t107;
        let t111 = 1.0 - 0.45341611111111111111e0 * t107;
        let t112 = 1.0 / t111;
        let t114 = t109 * t112 * t59;
        let t117 = t34 + 5.0 / 9.0 * t107 * t114;
        let t118 = 5.0 / 9.0 * t107;
        let t119 = 1.0 - t118;
        let t120 = Heaviside::<f64>(t119);
        let t122 = 1.0 + t118;
        let t123 = 1.0 / t122;
        let t126 = 1.0 + 0.148e0 * t119 * t123;
        let t127 = -t119;
        let t128 = Heaviside::<f64>(t127);
        let t130 = t117 * t120 + t126 * t128;
        let t133 = 1.0 + 0.1015549e-2 * t103;
        let t134 = 1.0 / t133;
        let t135 = t28 * t130 * t134;
        let t138 = piecewise3::<f64>(t85, 0.0, -3.0 / 8.0 * t94 * t135);
        let tzk0 = t84 + t138;
        zk[ip] += tzk0;
    }
}
