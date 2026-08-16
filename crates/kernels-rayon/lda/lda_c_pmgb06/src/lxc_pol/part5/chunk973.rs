//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 973/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk973(t2481: f64, t3213: f64, t2493: f64, t3220: f64, t132: f64, t1547: f64, t2605: f64, t4836: f64, t802: f64, t1554: f64, t161: f64, t2600: f64) -> (f64, f64, f64, f64, f64) {
    let t16152 = t3213 * t2481;
    let t16158 = t3220 * t2493;
    let t16161 = t132 * t1547 * t2605;
    let t16173 = t802 * t4836;
    let t16178 = t161 * t1554 * t2600;
    (t16152, t16158, t16161, t16173, t16178)
}
