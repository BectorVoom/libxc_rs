//! LDA_C_RC04 exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_rc04.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn lda_c_rc04_exc_pol(
    rho: &[f64],
    zk: &mut [f64],
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
        let t9 = pow_1_3(t5);
        let t10 = t9 * t9;
        let t11 = piecewise3(t6, t8, t10);
        let t12 = 1.0 - t4;
        let t13 = t12 <= zeta_threshold;
        let t14 = pow_1_3(t12);
        let t15 = t14 * t14;
        let t16 = piecewise3(t13, t8, t15);
        let t18 = t11 / 2.0 + t16 / 2.0;
        let t19 = t18 * t18;
        let t20 = t19 * t18;
        let t21 = M_CBRT3;
        let t23 = pow_1_3(1.0 / M_PI);
        let t24 = t21 * t23;
        let t25 = M_CBRT4;
        let t26 = t25 * t25;
        let t27 = pow_1_3(t2);
        let t32 = 4.88827 + 0.79425925 * t24 * t26 / t27;
        let t33 = f64::atan(t32);
        let t35 = -0.655868 * t33 + 0.897889;
        let t37 = t21 * t21;
        let t38 = t20 * t35 * t37;
        let t39 = 1.0 / t23;
        let t40 = t39 * t25;
        let t41 = t40 * t27;
        let t42 = t38 * t41;
        let tzk0 = t42 / 3.0;
        zk[ip] += tzk0;
    }
}
