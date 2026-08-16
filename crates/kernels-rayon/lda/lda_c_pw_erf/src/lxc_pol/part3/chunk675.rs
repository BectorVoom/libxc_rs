//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 675/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk675(t1726: f64, t405: f64, t1112: f64, t462: f64, t159: f64, t285: f64, t1159: f64, t477: f64, t1128: f64, t695: f64, t39: f64, t465: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4117 = t405 * t1726;
    let t4120 = t462 * t1112;
    let t4122 = t4120 * t159 * t285;
    let t4125 = t1159 * t477 * t285;
    let t4129 = 0.0008717022455366076_f64 * t695 * t1128 * t285;
    let t4130 = t39 * t465;
    (t4117, t4120, t4122, t4125, t4129, t4130)
}
