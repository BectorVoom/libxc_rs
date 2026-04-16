//! MGGA_X_PBE_GX exc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 50 shared lines across all orders.
//! Delta: 50 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_x_pbe_gx_exc_unpol(
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
        // --- shared preamble (50 lines) ---
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = M_CBRTPI;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3(zeta_threshold);
        let t16 = pow_1_3(t12);
        let t18 = piecewise3(t12 <= zeta_threshold, t14 * zeta_threshold, t16 * t12);
        let t19 = t4 / t5 * t18;
        let t20 = pow_1_3(rho[ip]);
        let t21 = M_CBRT2;
        let t22 = t4 * t4;
        let t24 = M_CBRT4;
        let t26 = 8.0 / 27.0 * t21 * t22 * t24;
        let t27 = t21 * t21;
        let t28 = tau[ip] * t27;
        let t29 = t20 * t20;
        let t31 = 1.0 / t29 / rho[ip];
        let t33 = sigma[ip] * t27;
        let t34 = rho[ip] * rho[ip];
        let t36 = 1.0 / t29 / t34;
        let t37 = t33 * t36;
        let t39 = t28 * t31 - t37 / 8.0;
        let t40 = M_CBRT6;
        let t42 = M_PI * M_PI;
        let t43 = pow_1_3(t42);
        let t44 = t43 * t43;
        let t45 = 1.0 / t44;
        let t46 = t39 * t40 * t45;
        let t48 = 0.827411e0 - 0.35753333333333333333e0 * t46;
        let t50 = 1.0 - 0.45341611111111111111e0 * t46;
        let t51 = 1.0 / t50;
        let t53 = 1.0 - t26;
        let t54 = t48 * t51 * t53;
        let t57 = t26 + 5.0 / 9.0 * t46 * t54;
        let t58 = 5.0 / 9.0 * t46;
        let t59 = 1.0 - t58;
        let t60 = Heaviside(t59);
        let t62 = 1.0 + t58;
        let t63 = 1.0 / t62;
        let t66 = 1.0 + 0.148e0 * t59 * t63;
        let t67 = -t59;
        let t68 = Heaviside(t67);
        let t70 = t57 * t60 + t66 * t68;
        let t73 = 1.0 + 0.1015549e-2 * t37;
        let t74 = 1.0 / t73;
        let t78 = piecewise3(t3, 0.0, -3.0 / 8.0 * t19 * t20 * t70 * t74);
        let tzk0 = 2.0 * t78;
        zk[ip] += tzk0;
    }
}
