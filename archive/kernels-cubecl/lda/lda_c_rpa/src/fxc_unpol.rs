//! LDA_C_RPA fxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_rpa.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3};

/// LDA_C_RPA fxc -- unpolarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_rpa_fxc_unpol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t1 = M_CBRT3;
        let t3 = pow_1_3::<f64>(1.0 / M_PI);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = pow_1_3::<f64>(rho[ip]);
        let t9 = t6 / t7;
        let t10 = t4 * t9;
        let t12 = f64::ln(t10 / 4.0);
        let t13 = 0.0311 * t12;
        let t16 = 0.00225 * t4 * t9 * t12;
        let t17 = 0.00425 * t10;
        let tzk0 = t13 - 0.048 + t16 - t17;
        zk[ip] += tzk0;
        let t18 = 1.0 / rho[ip];
        let t22 = t6 / t7 / rho[ip];
        let t24 = t4 * t22 * t12;
        let t26 = t4 * t22;
        let tvrho0 = t13 - 0.048 + t16 - t17 + rho[ip] * (-0.010366666666666666 * t18 - 0.00075 * t24 + 0.0006666666666666666 * t26);
        vrho[ip] += tvrho0;
        let t33 = rho[ip] * rho[ip];
        let t34 = 1.0 / t33;
        let t38 = t6 / t7 / t33;
        let t40 = t4 * t38 * t12;
        let t42 = t4 * t38;
        let tv2rho20 = -0.020733333333333333 * t18 - 0.0015 * t24 + 0.0013333333333333333 * t26 + rho[ip] * (0.010366666666666666 * t34 + 0.001 * t40 - 0.0006388888888888889 * t42);
        v2rho2[ip] += tv2rho20;
    }
}
