//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 756/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk756(t34: f64, t581: f64, t593: f64, t4892: f64, t1318: f64, t1336: f64, t2146: f64, t1124: f64, t573: f64, t2152: f64, t571: f64, t1446: f64, t2143: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4893 = t581 * t34;
    let t4894 = t4893 * t593;
    let t4895 = t4892 * t4894;
    let t4897 = 8.0_f64 / 15.0_f64 * t1318 * t4895;
    let t4899 = 8.0_f64 / 45.0_f64 * t2146 * t1336;
    let t4900 = t1124 * t573;
    let t4901 = t4900 * t2152;
    let t4902 = t571 * t4901;
    let t4903 = 8.0_f64 / 27.0_f64 * t4902;
    let t4905 = 16.0_f64 / 135.0_f64 * t1446 * t2143;
    (t4893, t4894, t4895, t4897, t4899, t4900, t4901, t4903, t4905)
}
