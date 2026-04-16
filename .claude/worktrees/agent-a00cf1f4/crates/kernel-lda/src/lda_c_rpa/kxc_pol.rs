//! LDA_C_RPA kxc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 14 shared lines across all orders.
//! Delta: 9 lines unique to kxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3};

/// LDA_C_RPA kxc -- polarized (incremental).
#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn lda_c_rpa_kxc_pol(
    rho: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    v2rho2: &mut Array<f64>,
    v3rho3: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        // --- shared preamble (14 lines) ---
        let t1 = M_CBRT3;
        let t3 = pow_1_3(1.0 / M_PI);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = rho0 + rho1;
        let t8 = pow_1_3(t7);
        let t10 = t6 / t8;
        let t11 = t4 * t10;
        let t13 = f64::ln(t11 / 4.0);
        let t14 = 0.0311 * t13;
        let t17 = 0.00225 * t4 * t10 * t13;
        let t18 = 0.00425 * t11;
        let tzk0 = t14 - 0.048 + t17 - t18;
        zk[ip] += tzk0;
        // --- vxc delta (6 lines) ---
        let t19 = 1.0 / t7;
        let t23 = t6 / t8 / t7;
        let t25 = t4 * t23 * t13;
        let t27 = t4 * t23;
        let tvrho0 = t14 - 0.048 + t17 - t18 + t7 * (-0.010366666666666666 * t19 - 0.00075 * t25 + 0.0006666666666666666 * t27);
        vrho[ip * 2] += tvrho0;
        let tvrho1 = tvrho0;
        vrho[ip * 2 + 1] += tvrho1;
        // --- fxc delta (8 lines) ---
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
        // --- kxc delta (this level) (9 lines) ---
        let t50 = t34 * t7;
        let t51 = 1.0 / t50;
        let t55 = t6 / t8 / t50;
        let t57 = t4 * t55 * t13;
        let t59 = t4 * t55;
        let tv3rho30 = 0.0311 * t35 + 0.003 * t41 - 0.0019166666666666666 * t43 + t7 * (-0.020733333333333333 * t51 - 0.0023333333333333335 * t57 + 0.0011574074074074073 * t59);
        v3rho3[ip * 4] += tv3rho30;
        let tv3rho31 = tv3rho30;
        v3rho3[ip * 4 + 1] += tv3rho31;
        let tv3rho32 = tv3rho31;
        v3rho3[ip * 4 + 2] += tv3rho32;
        let tv3rho33 = tv3rho32;
        v3rho3[ip * 4 + 3] += tv3rho33;
    }
}
