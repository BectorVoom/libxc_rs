//! MGGA_X_MS exc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_ms.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn mgga_x_ms_exc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    param_b: f64,
    param_c: f64,
    param_kappa: f64,
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
        let t29 = M_CBRT6;
        let t30 = M_PI * M_PI;
        let t31 = pow_1_3::<f64>(t30);
        let t32 = t31 * t31;
        let t33 = 1.0 / t32;
        let t34 = t29 * t33;
        let t35 = rho0 * rho0;
        let t36 = pow_1_3::<f64>(rho0);
        let t37 = t36 * t36;
        let t39 = 1.0 / t37 / t35;
        let t40 = sigma0 * t39;
        let t42 = 5.0 / 972.0 * t34 * t40;
        let t43 = param_kappa + t42;
        let t47 = param_kappa * (1.0 - param_kappa / t43);
        let t49 = 1.0 / t37 / rho0;
        let t52 = tau0 * t49 - t40 / 8.0;
        let t53 = t52 * t52;
        let t54 = t29 * t29;
        let t57 = 1.0 / t31 / t30;
        let t60 = 1.0 - 25.0 / 81.0 * t53 * t54 * t57;
        let t61 = t60 * t60;
        let t62 = t61 * t60;
        let t63 = t53 * t52;
        let t64 = t30 * t30;
        let t65 = 1.0 / t64;
        let t68 = t53 * t53;
        let t71 = t64 * t64;
        let t72 = 1.0 / t71;
        let t75 = 1.0 + 250.0 / 243.0 * t63 * t65 + 62500.0 / 59049.0 * param_b * t68 * t53 * t72;
        let t76 = 1.0 / t75;
        let t77 = t62 * t76;
        let t78 = param_kappa + t42 + param_c;
        let t83 = param_kappa * (1.0 - param_kappa / t78) - t47;
        let t85 = t77 * t83 + t47 + 1.0;
        let t89 = piecewise3::<f64>(t2, 0.0, -3.0 / 8.0 * t6 * t28 * t85);
        let t90 = rho1 <= dens_threshold;
        let t91 = -t17;
        let t93 = piecewise5::<f64>(t15, t12, t11, t16, t91 * t8);
        let t94 = 1.0 + t93;
        let t95 = t94 <= zeta_threshold;
        let t96 = pow_1_3::<f64>(t94);
        let t98 = piecewise3::<f64>(t95, t23, t96 * t94);
        let t99 = t98 * t27;
        let t100 = rho1 * rho1;
        let t101 = pow_1_3::<f64>(rho1);
        let t102 = t101 * t101;
        let t104 = 1.0 / t102 / t100;
        let t105 = sigma2 * t104;
        let t107 = 5.0 / 972.0 * t34 * t105;
        let t108 = param_kappa + t107;
        let t112 = param_kappa * (1.0 - param_kappa / t108);
        let t114 = 1.0 / t102 / rho1;
        let t117 = tau1 * t114 - t105 / 8.0;
        let t118 = t117 * t117;
        let t122 = 1.0 - 25.0 / 81.0 * t118 * t54 * t57;
        let t123 = t122 * t122;
        let t124 = t123 * t122;
        let t125 = t118 * t117;
        let t128 = t118 * t118;
        let t133 = 1.0 + 250.0 / 243.0 * t125 * t65 + 62500.0 / 59049.0 * param_b * t128 * t118 * t72;
        let t134 = 1.0 / t133;
        let t135 = t124 * t134;
        let t136 = param_kappa + t107 + param_c;
        let t141 = param_kappa * (1.0 - param_kappa / t136) - t112;
        let t143 = t135 * t141 + t112 + 1.0;
        let t147 = piecewise3::<f64>(t90, 0.0, -3.0 / 8.0 * t6 * t99 * t143);
        let tzk0 = t89 + t147;
        zk[ip] += tzk0;
    }
}
