//! LDA_X_2D exc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_x_2d.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};

/// LDA_X_2D exc -- polarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_x_2d_exc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
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
    }
}
