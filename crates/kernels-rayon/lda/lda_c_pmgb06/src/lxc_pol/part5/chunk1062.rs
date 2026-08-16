//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1062/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1062(t16181: f64, t6600: f64, t802: f64, t6572: f64, t1887: f64, t2631: f64, t161: f64, t166: f64, t2623: f64, t4801: f64, t1908: f64, t6127: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19712 = t16181 / 15.0_f64;
    let t19714 = t802 * t6600 / 5.0_f64;
    let t19716 = t802 * t6572 / 5.0_f64;
    let t19718 = t1887 * t2631 / 5.0_f64;
    let t19722 = t161 * t166 * t4801 * t2623 / 10.0_f64;
    let t19724 = t6127 * t1908 / 15.0_f64;
    (t19712, t19714, t19716, t19718, t19722, t19724)
}
