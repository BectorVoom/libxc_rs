//! MGGA_X_PKZB exc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_pkzb.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_x_pkzb_exc_unpol(
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
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = M_CBRTPI;
        let t7 = t4 / t5;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3(zeta_threshold);
        let t16 = pow_1_3(t12);
        let t18 = piecewise3(t12 <= zeta_threshold, t14 * zeta_threshold, t16 * t12);
        let t19 = pow_1_3(rho[ip]);
        let t21 = M_CBRT6;
        let t22 = M_PI * M_PI;
        let t23 = pow_1_3(t22);
        let t24 = t23 * t23;
        let t25 = 1.0 / t24;
        let t26 = t21 * t25;
        let t27 = M_CBRT2;
        let t28 = t27 * t27;
        let t29 = sigma[ip] * t28;
        let t30 = rho[ip] * rho[ip];
        let t31 = t19 * t19;
        let t33 = 1.0 / t31 / t30;
        let t34 = t29 * t33;
        let t35 = t26 * t34;
        let t37 = tau[ip] * t28;
        let t39 = 1.0 / t31 / rho[ip];
        let t44 = t26 * t37 * t39 / 4.0 - 9.0 / 20.0 - t35 / 288.0;
        let t45 = t44 * t44;
        let t47 = t44 * t21;
        let t48 = t47 * t25;
        let t51 = t21 * t21;
        let t53 = 1.0 / t23 / t22;
        let t54 = t51 * t53;
        let t55 = sigma[ip] * sigma[ip];
        let t56 = t55 * t27;
        let t57 = t30 * t30;
        let t58 = t57 * rho[ip];
        let t60 = 1.0 / t19 / t58;
        let t64 = 0.804e0 + 5.0 / 972.0 * t35 + 146.0 / 2025.0 * t45 - 73.0 / 9720.0 * t48 * t34 + 0.45818468001825619316e-3 * t54 * t56 * t60;
        let t67 = 0.1804e1 - 0.646416e0 / t64;
        let t71 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t18 * t19 * t67);
        let tzk0 = 2.0 * t71;
        zk[ip] += tzk0;
    }
}
