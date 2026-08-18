//! LDA_C_RPA lxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_rpa.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn lda_c_rpa_lxc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    v3rho3: &mut [f64],
    v4rho4: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t1 = M_CBRT3;
        let t3 = pow_1_3(1.0 / M_PI);
        let t4 = t1 * t3;
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = pow_1_3(rho[ip]);
        let t9 = t6 / t7;
        let t10 = t4 * t9;
        let t12 = f64::ln(t10 / 4.0);
        let t13 = 0.0311 * t12;
        let t16 = 0.00225 * t4 * t9 * t12;
        let t17 = 0.00425 * t10;
        let tzk0 = t13 - 0.048 + t16 - t17;
        zk[ip] += tzk0;
        let t18 = 1.0 / rho[ip];
        let t22 = t6 / t7 / rho[ip];
        let t24 = t4 * t22 * t12;
        let t26 = t4 * t22;
        let tvrho0 = t13 - 0.048 + t16 - t17 + rho[ip] * (-0.010366666666666666 * t18 - 0.00075 * t24 + 0.0006666666666666666 * t26);
        vrho[ip] += tvrho0;
        let t33 = rho[ip] * rho[ip];
        let t34 = 1.0 / t33;
        let t38 = t6 / t7 / t33;
        let t40 = t4 * t38 * t12;
        let t42 = t4 * t38;
        let tv2rho20 = -0.020733333333333333 * t18 - 0.0015 * t24 + 0.0013333333333333333 * t26 + rho[ip] * (0.010366666666666666 * t34 + 0.001 * t40 - 0.0006388888888888889 * t42);
        v2rho2[ip] += tv2rho20;
        let t49 = t33 * rho[ip];
        let t50 = 1.0 / t49;
        let t54 = t6 / t7 / t49;
        let t56 = t4 * t54 * t12;
        let t58 = t4 * t54;
        let tv3rho30 = 0.0311 * t34 + 0.003 * t40 - 0.0019166666666666666 * t42 + rho[ip] * (-0.020733333333333333 * t50 - 0.0023333333333333335 * t56 + 0.0011574074074074073 * t58);
        v3rho3[ip] += tv3rho30;
        let t65 = t33 * t33;
        let t70 = t6 / t7 / t65;
        let tv4rho40 = -0.08293333333333333 * t50 - 0.009333333333333334 * t56 + 0.004629629629629629 * t58 + rho[ip] * (0.0622 / t65 + 0.0077777777777777776 * t4 * t70 * t12 - 0.003080246913580247 * t4 * t70);
        v4rho4[ip] += tv4rho40;
    }
}
