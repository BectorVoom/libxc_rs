//! LDA_K_ZLP exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_k_zlp.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn lda_k_zlp_exc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t1 = M_CBRT3;
        let t2 = t1 * t1;
        let t4 = pow_1_3(1.0 / M_PI);
        let t5 = 1.0 / t4;
        let t7 = M_CBRT4;
        let t8 = t2 * t5 * t7;
        let t10 = pow_1_3(zeta_threshold);
        let t11 = t10 * t10;
        let t13 = piecewise3(1.0 <= zeta_threshold, t11 * zeta_threshold, 1.0);
        let t14 = pow_1_3(rho[ip]);
        let t15 = t14 * t14;
        let t16 = t13 * t15;
        let t17 = 1.0 / t14;
        let t19 = 1.0 + 510.2040816326531 * t17;
        let t20 = rmath::ln(t19);
        let t23 = 1.0 - 0.00196 * t14 * t20;
        let t25 = t8 * t16 * t23;
        let tzk0 = 1.0790666666666666 * t25;
        zk[ip] += tzk0;
    }
}
