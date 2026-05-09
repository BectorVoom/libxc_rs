//! GGA_K_DK exc unpol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 55 shared lines across all orders.
//! Delta: 55 lines unique to exc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn gga_k_dk_exc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    zk: &mut Array<f64>,
    param_aa_0: f64,
    param_aa_1: f64,
    param_aa_2: f64,
    param_aa_3: f64,
    param_aa_4: f64,
    param_bb_0: f64,
    param_bb_1: f64,
    param_bb_2: f64,
    param_bb_3: f64,
    param_bb_4: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        // --- shared preamble (55 lines) ---
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = t3 * t3;
        let t5 = M_CBRTPI;
        let t7 = t4 * t5 * M_PI;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3(zeta_threshold);
        let t15 = t14 * t14;
        let t17 = pow_1_3(t12);
        let t18 = t17 * t17;
        let t20 = piecewise3(t12 <= zeta_threshold, t15 * zeta_threshold, t18 * t12);
        let t21 = t7 * t20;
        let t22 = pow_1_3(rho[ip]);
        let t23 = t22 * t22;
        let t25 = param_aa_1;
        let t26 = t25 * sigma[ip];
        let t27 = M_CBRT2;
        let t28 = t27 * t27;
        let t29 = rho[ip] * rho[ip];
        let t31 = 1.0 / t23 / t29;
        let t32 = t28 * t31;
        let t34 = param_aa_2;
        let t35 = sigma[ip] * sigma[ip];
        let t36 = t34 * t35;
        let t37 = t29 * t29;
        let t38 = t37 * rho[ip];
        let t40 = 1.0 / t22 / t38;
        let t41 = t27 * t40;
        let t44 = param_aa_3;
        let t45 = t35 * sigma[ip];
        let t46 = t44 * t45;
        let t47 = t37 * t37;
        let t48 = 1.0 / t47;
        let t51 = param_aa_4;
        let t52 = t35 * t35;
        let t53 = t51 * t52;
        let t54 = t47 * t29;
        let t57 = t28 / t23 / t54;
        let t60 = t26 * t32 + 2.0 * t36 * t41 + 4.0 * t46 * t48 + 4.0 * t53 * t57 + param_aa_0;
        let t61 = t23 * t60;
        let t63 = param_bb_1;
        let t64 = t63 * sigma[ip];
        let t66 = param_bb_2;
        let t67 = t66 * t35;
        let t70 = param_bb_3;
        let t71 = t70 * t45;
        let t74 = param_bb_4;
        let t75 = t74 * t52;
        let t78 = t64 * t32 + 2.0 * t67 * t41 + 4.0 * t71 * t48 + 4.0 * t75 * t57 + param_bb_0;
        let t79 = 1.0 / t78;
        let t83 = piecewise3(t2, 0.0, 3.0 / 20.0 * t21 * t61 * t79);
        let tzk0 = 2.0 * t83;
        zk[ip] += tzk0;
    }
}
