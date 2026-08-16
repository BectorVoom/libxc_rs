//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 1038/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk1038(t31045: f64, t3564: f64, t13148: f64, t2083: f64, t7736: f64, t13153: f64, t2191: f64, t30294: f64, t5895: f64, t1421: f64, t19163: f64, t19235: f64, t26632: f64, t26692: f64, t31034: f64, t31038: f64, t31042: f64) -> f64 {
    let t31046 = t3564 * t31045;
    let t31050 = t13148 * t7736 * t2083;
    let t31054 = t13153 * t7736 * t2191;
    let t31057 = t5895 * t30294;
    let t31060 = 0.39422578e-2_f64 * t26632 - 0.98556445e-3_f64 * t19163 - 0.26281718666666666667e-2_f64 * t26692 + 0.65704296666666666665e-3_f64 * t19235 - 0.65704296666666666666e-2_f64 * t1421 * t31034 + 0.39422577999999999999e-2_f64 * t1421 * t31038 - 0.4435040025e-2_f64 * t1421 * t31042 - 0.4435040025e-2_f64 * t1421 * t31046 + 0.49278222499999999999e-2_f64 * t1421 * t31050 - 0.32852148333333333333e-2_f64 * t1421 * t31054 + 0.32852148333333333333e-2_f64 * t1421 * t31057;
    t31060
}
