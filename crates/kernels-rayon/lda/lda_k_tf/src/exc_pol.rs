//! LDA_K_TF exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_k_tf.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn lda_k_tf_exc_pol(
    rho: &[f64],
    zk: &mut [f64],
    param_ax: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t1 = rho0 - rho1;
        let t2 = rho0 + rho1;
        let t3 = 1.0 / t2;
        let t4 = t1 * t3;
        let t5 = 1.0 + t4;
        let t6 = t5 <= zeta_threshold;
        let t7 = pow_1_3(zeta_threshold);
        let t8 = t7 * t7;
        let t9 = t8 * zeta_threshold;
        let t10 = pow_1_3(t5);
        let t11 = t10 * t10;
        let t13 = piecewise3(t6, t9, t11 * t5);
        let t14 = 1.0 - t4;
        let t15 = t14 <= zeta_threshold;
        let t16 = pow_1_3(t14);
        let t17 = t16 * t16;
        let t19 = piecewise3(t15, t9, t17 * t14);
        let t23 = M_CBRT3;
        let t24 = param_ax * (t13 / 2.0 + t19 / 2.0) * t23;
        let t26 = pow_1_3(1.0 / M_PI);
        let t27 = t26 * t26;
        let t28 = 1.0 / t27;
        let t29 = M_CBRT4;
        let t30 = t29 * t29;
        let t31 = t28 * t30;
        let t32 = pow_1_3(t2);
        let t33 = t32 * t32;
        let t34 = t31 * t33;
        let t35 = t24 * t34;
        let tzk0 = t35 / 3.0;
        zk[ip] += tzk0;
    }
}
