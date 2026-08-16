//! LDA_C_LP96 kxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_lp96.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::powers::{pow_1_3};

/// LDA_C_LP96 kxc -- polarized.
#[allow(unused_variables, non_snake_case)]
pub fn lda_c_lp96_kxc_pol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    v3rho3: &mut [f64],
    param_C1: f64,
    param_C2: f64,
    param_C3: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = rho0 + rho1;
        let t2 = pow_1_3(t1);
        let t4 = param_C2 / t2;
        let t5 = t2 * t2;
        let t7 = param_C3 / t5;
        let tzk0 = param_C1 + t4 + t7;
        zk[ip] += tzk0;
        let t10 = param_C2 / t2 / t1;
        let t14 = param_C3 / t5 / t1;
        let tvrho0 = param_C1 + t4 + t7 + t1 * (-t10 / 3.0 - 2.0 / 3.0 * t14);
        vrho[ip * 2] += tvrho0;
        let tvrho1 = tvrho0;
        vrho[ip * 2 + 1] += tvrho1;
        let t20 = t1 * t1;
        let t23 = param_C2 / t2 / t20;
        let t27 = param_C3 / t5 / t20;
        let tv2rho20 = -2.0 / 3.0 * t10 - 4.0 / 3.0 * t14 + t1 * (4.0 / 9.0 * t23 + 10.0 / 9.0 * t27);
        v2rho2[ip * 3] += tv2rho20;
        let tv2rho21 = tv2rho20;
        v2rho2[ip * 3 + 1] += tv2rho21;
        let tv2rho22 = tv2rho21;
        v2rho2[ip * 3 + 2] += tv2rho22;
        let t33 = t20 * t1;
        let t36 = param_C2 / t2 / t33;
        let t40 = param_C3 / t5 / t33;
        let tv3rho30 = 4.0 / 3.0 * t23 + 10.0 / 3.0 * t27 + t1 * (-28.0 / 27.0 * t36 - 80.0 / 27.0 * t40);
        v3rho3[ip * 4] += tv3rho30;
        let tv3rho31 = tv3rho30;
        v3rho3[ip * 4 + 1] += tv3rho31;
        let tv3rho32 = tv3rho31;
        v3rho3[ip * 4 + 2] += tv3rho32;
        let tv3rho33 = tv3rho32;
        v3rho3[ip * 4 + 3] += tv3rho33;
    }
}
