//! LDA_X_2D vxc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 24 shared lines across all orders.
//! Delta: 18 lines unique to vxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};

/// LDA_X_2D vxc -- polarized (incremental).
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_x_2d_vxc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        // --- shared preamble (24 lines) ---
        let t1 = M_SQRT2;
        let t2 = f64::sqrt(M_PI);
        let t3 = 1.0 / t2;
        let t4 = t1 * t3;
        let t5 = rho0 - rho1;
        let t6 = rho0 + rho1;
        let t7 = 1.0 / t6;
        let t8 = t5 * t7;
        let t9 = 1.0 + t8;
        let t10 = t9 <= zeta_threshold;
        let t11 = f64::sqrt(zeta_threshold);
        let t12 = t11 * zeta_threshold;
        let t13 = f64::sqrt(t9);
        let t14 = t13 * t9;
        let t15 = piecewise3(t10, t12, t14);
        let t16 = 1.0 - t8;
        let t17 = t16 <= zeta_threshold;
        let t18 = f64::sqrt(t16);
        let t19 = t18 * t16;
        let t20 = piecewise3(t17, t12, t19);
        let t22 = t15 / 2.0 + t20 / 2.0;
        let t23 = f64::sqrt(t6);
        let t25 = t4 * t22 * t23;
        let tzk0 = -4.0 / 3.0 * t25;
        zk[ip] += tzk0;
        // --- vxc delta (this level) (18 lines) ---
        let t27 = 2.0 * t25;
        let t28 = t23 * t6;
        let t29 = t28 * t1;
        let t30 = t6 * t6;
        let t31 = 1.0 / t30;
        let t32 = t5 * t31;
        let t33 = t7 - t32;
        let t36 = piecewise3(t10, 0.0, 3.0 / 2.0 * t13 * t33);
        let t37 = -t33;
        let t40 = piecewise3(t17, 0.0, 3.0 / 2.0 * t18 * t37);
        let t42 = t36 / 2.0 + t40 / 2.0;
        let tvrho0 = -t27 - 4.0 / 3.0 * t29 * t3 * t42;
        vrho[ip * 2] += tvrho0;
        let t46 = -t7 - t32;
        let t49 = piecewise3(t10, 0.0, 3.0 / 2.0 * t13 * t46);
        let t50 = -t46;
        let t53 = piecewise3(t17, 0.0, 3.0 / 2.0 * t18 * t50);
        let t56 = t3 * (t49 / 2.0 + t53 / 2.0);
        let tvrho1 = -t27 - 4.0 / 3.0 * t29 * t56;
        vrho[ip * 2 + 1] += tvrho1;
    }
}
