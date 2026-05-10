//! LDA_XC_ZLP kxc pol kernel (incremental).
//!
//! Auto-translated with incremental derivative structure.
//! Preamble: 7 shared lines across all orders.
//! Delta: 9 lines unique to kxc.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::powers::{pow_1_3};

/// LDA_XC_ZLP kxc -- polarized (incremental).
#[allow(unused_variables, non_snake_case)]
#[cube]
pub fn lda_xc_zlp_kxc_pol(
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
        // --- shared preamble (7 lines) ---
        let t1 = rho0 + rho1;
        let t2 = pow_1_3(t1);
        let t5 = 1.0 + 105.5562709925034 / t2;
        let t6 = f64::ln(t5);
        let t9 = 1.0 - 0.00947362 * t6 * t2;
        let t10 = t9 * t2;
        let tzk0 = -0.93222 * t10;
        zk[ip] += tzk0;
        // --- vxc delta (7 lines) ---
        let t13 = t2 * t1;
        let t15 = 1.0 / t5;
        let t18 = t2 * t2;
        let t19 = 1.0 / t18;
        let t22 = 0.3333333333333333 / t1 * t15 - 0.0031578733333333334 * t6 * t19;
        let tvrho0 = -1.24296 * t10 - 0.93222 * t13 * t22;
        vrho[ip * 2] += tvrho0;
        let tvrho1 = tvrho0;
        vrho[ip * 2 + 1] += tvrho1;
        // --- fxc delta (8 lines) ---
        let t29 = t1 * t1;
        let t35 = t5 * t5;
        let t36 = 1.0 / t35;
        let t40 = 1.0 / t18 / t1;
        let t43 = -0.2222222222222222 / t29 * t15 + 11.728474554722599 / t2 / t29 * t36 + 0.002105248888888889 * t6 * t40;
        let tv2rho20 = -2.48592 * t22 * t2 - 0.41432 * t9 * t19 - 0.93222 * t13 * t43;
        v2rho2[ip * 3] += tv2rho20;
        let tv2rho21 = tv2rho20;
        v2rho2[ip * 3 + 1] += tv2rho21;
        let tv2rho22 = tv2rho21;
        v2rho2[ip * 3 + 2] += tv2rho22;
        // --- kxc delta (this level) (9 lines) ---
        let t52 = t29 * t1;
        let t61 = 1.0 / t18 / t52;
        let t63 = 1.0 / t35 / t5;
        let t67 = 1.0 / t18 / t29;
        let t70 = 0.37037037037037035 / t52 * t15 - 35.1854236641678 / t2 / t52 * t36 + 825.3426922846528 * t61 * t63 - 0.003508748148148148 * t6 * t67;
        let tv3rho30 = -3.72888 * t43 * t2 - 1.24296 * t22 * t19 + 0.2762133333333333 * t9 * t40 - 0.93222 * t13 * t70;
        v3rho3[ip * 4] += tv3rho30;
        let tv3rho31 = tv3rho30;
        v3rho3[ip * 4 + 1] += tv3rho31;
        let tv3rho32 = tv3rho31;
        v3rho3[ip * 4 + 2] += tv3rho32;
        let tv3rho33 = tv3rho32;
        v3rho3[ip * 4 + 3] += tv3rho33;
    }
}
