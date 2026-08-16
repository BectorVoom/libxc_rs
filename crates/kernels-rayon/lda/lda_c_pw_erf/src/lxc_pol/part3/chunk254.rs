//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 254/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk254(t143: f64, t756: f64, t120: f64, t102: f64, t128: f64) -> (f64, f64, f64, f64) {
    let t757 = t143 * t756;
    let t760 = t120 * t756;
    let t762 = 2.923025_f64 * t102 * t760;
    let t763 = t128 * t756;
    (t757, t760, t762, t763)
}
