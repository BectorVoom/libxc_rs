//! LDA_C_ML1 exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_ml1.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn lda_c_ml1_exc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    param_fc: f64,
    param_q: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t1 = 1.0 <= zeta_threshold;
        let t2 = pow_1_3(rho[ip]);
        let t4 = zeta_threshold - 1.0;
        let t6 = piecewise5(t1, t4, t1, -t4, 0.0);
        let t7 = 1.0 + t6;
        let t8 = rmath::pow(t7, param_q);
        let t9 = 1.0 - t6;
        let t10 = rmath::pow(t9, param_q);
        let t11 = t8 + t10;
        let t12 = t6 * t6;
        let t13 = 1.0 - t12;
        let t14 = pow_1_3(t13);
        let t16 = pow_1_3(t7);
        let t17 = pow_1_3(t9);
        let t18 = t16 + t17;
        let t20 = t11 * t14 / t18;
        let t23 = 1.0 + 10.874334072525 * t2 * param_fc * t20;
        let t26 = 1.0 / t2;
        let t27 = 1.0 / param_fc;
        let t32 = 1.0 / t11 / t14 * t18;
        let t33 = t26 * t27 * t32;
        let t35 = 1.0 + 0.09195962397381102 * t33;
        let t36 = rmath::ln(t35);
        let t42 = t2 * t2;
        let t43 = 1.0 / t42;
        let t44 = param_fc * param_fc;
        let t45 = 1.0 / t44;
        let t47 = t11 * t11;
        let t48 = 1.0 / t47;
        let t49 = t14 * t14;
        let t50 = 1.0 / t49;
        let t52 = t18 * t18;
        let t53 = t48 * t50 * t52;
        let t57 = piecewise3(t1, 0.0, -0.69079225 / t23 + 0.07036135105016941 * t36 * t26 * t27 * t32 + 0.0635250071315033 * t33 - 0.012312144854458484 * t43 * t45 * t53);
        let tzk0 = rho[ip] * t57;
        zk[ip] += tzk0;
    }
}
