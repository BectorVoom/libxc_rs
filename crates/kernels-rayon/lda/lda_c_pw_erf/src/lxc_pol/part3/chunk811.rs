//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 811/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk811(t102: f64, t1568: f64, t763: f64, t1852: f64, t431: f64, t156: f64, t4: f64, t411: f64, t1840: f64, t426: f64, t1856: f64, t767: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5591 = 5.84605_f64 * t102 * t763 * t1568;
    let t5592 = t431 * t1852;
    let t5594 = t4 * t156 * t411;
    let t5596 = 5.87616_f64 * t5592 * t5594;
    let t5598 = t426 * t156 * t1840;
    let t5599 = t1856 * t411;
    let t5603 = t767 * t1568;
    (t5591, t5592, t5594, t5596, t5598, t5599, t5603)
}
