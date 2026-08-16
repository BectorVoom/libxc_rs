//! LDA_C_RPA fxc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_rpa.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3};

/// LDA_C_RPA fxc -- polarized.
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_rpa_fxc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = M_CBRT3;
        let t3 = pow_1_3::<f64>(1.0 / M_PI);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = rho0 + rho1;
        let t8 = pow_1_3::<f64>(t7);
        let t10 = t6 / t8;
        let t11 = t4 * t10;
        let t13 = f64::ln(t11 / 4.0);
        let t14 = 0.0311 * t13;
        let t17 = 0.00225 * t4 * t10 * t13;
        let t18 = 0.00425 * t11;
        let tzk0 = t14 - 0.048 + t17 - t18;
        zk[ip] += tzk0;
        let t19 = 1.0 / t7;
        let t23 = t6 / t8 / t7;
        let t25 = t4 * t23 * t13;
        let t27 = t4 * t23;
        let tvrho0 = t14 - 0.048 + t17 - t18 + t7 * (-0.010366666666666666 * t19 - 0.00075 * t25 + 0.0006666666666666666 * t27);
        vrho[ip * 2] += tvrho0;
        let tvrho1 = tvrho0;
        vrho[ip * 2 + 1] += tvrho1;
        let t34 = t7 * t7;
        let t35 = 1.0 / t34;
        let t39 = t6 / t8 / t34;
        let t41 = t4 * t39 * t13;
        let t43 = t4 * t39;
        let tv2rho20 = -0.020733333333333333 * t19 - 0.0015 * t25 + 0.0013333333333333333 * t27 + t7 * (0.010366666666666666 * t35 + 0.001 * t41 - 0.0006388888888888889 * t43);
        v2rho2[ip * 3] += tv2rho20;
        let tv2rho21 = tv2rho20;
        v2rho2[ip * 3 + 1] += tv2rho21;
        let tv2rho22 = tv2rho21;
        v2rho2[ip * 3 + 2] += tv2rho22;
    }
}
