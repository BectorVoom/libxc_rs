//! LDA_XC_ZLP vxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_xc_zlp.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn lda_xc_zlp_vxc_pol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = rho0 + rho1;
        let t2 = pow_1_3(t1);
        let t5 = 1.0 + 105.5562709925034 / t2;
        let t6 = f64::ln(t5);
        let t9 = 1.0 - 0.00947362 * t6 * t2;
        let t10 = t9 * t2;
        let tzk0 = -0.93222 * t10;
        zk[ip] += tzk0;
        let t13 = t2 * t1;
        let t15 = 1.0 / t5;
        let t18 = t2 * t2;
        let t19 = 1.0 / t18;
        let t22 = 0.3333333333333333 / t1 * t15 - 0.0031578733333333334 * t6 * t19;
        let tvrho0 = -1.24296 * t10 - 0.93222 * t13 * t22;
        vrho[ip * 2] += tvrho0;
        let tvrho1 = tvrho0;
        vrho[ip * 2 + 1] += tvrho1;
    }
}
