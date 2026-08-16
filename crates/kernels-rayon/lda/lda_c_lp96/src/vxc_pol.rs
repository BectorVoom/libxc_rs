//! LDA_C_LP96 vxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_lp96.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::powers::{pow_1_3};

/// LDA_C_LP96 vxc -- polarized.
#[allow(unused_variables, non_snake_case)]
pub fn lda_c_lp96_vxc_pol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
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
    }
}
