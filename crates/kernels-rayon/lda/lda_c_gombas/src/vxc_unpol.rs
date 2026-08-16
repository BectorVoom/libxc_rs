//! LDA_C_GOMBAS vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_gombas.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::powers::{pow_1_3};

/// LDA_C_GOMBAS vxc -- unpolarized.
#[allow(unused_variables, non_snake_case)]
pub fn lda_c_gombas_vxc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t1 = pow_1_3(rho[ip]);
        let t2 = 1.0 / t1;
        let t4 = 1.0 + 0.0562 * t2;
        let t6 = 0.0357 / t4;
        let t7 = t2 + 2.39;
        let t9 = f64::ln(t7 * t1);
        let t10 = 0.0311 * t9;
        let tzk0 = -t6 - t10;
        zk[ip] += tzk0;
        let t11 = t4 * t4;
        let t12 = 1.0 / t11;
        let t14 = 1.0 / t1 / rho[ip];
        let t15 = t12 * t14;
        let t18 = t1 * t1;
        let t22 = -1.0 / rho[ip] / 3.0 + t7 / t18 / 3.0;
        let t23 = 1.0 / t7;
        let t24 = t22 * t23;
        let t25 = t24 * t2;
        let tvrho0 = -t6 - t10 + rho[ip] * (-0.00066878 * t15 - 0.0311 * t25);
        vrho[ip] += tvrho0;
    }
}
