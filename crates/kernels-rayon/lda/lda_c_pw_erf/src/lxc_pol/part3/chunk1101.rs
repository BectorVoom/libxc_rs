//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1101/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1101(t5064: f64, t518: f64, t2007: f64, t12641: f64, t2146: f64, t3829: f64, t3421: f64, t4763: f64, t3864: f64, t3819: f64, t4738: f64, t3982: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12881 = t5064 * t518;
    let t12883 = 8.0_f64 / 15.0_f64 * t12881 * t2007;
    let t12885 = 16.0_f64 / 15.0_f64 * t12641 * t2007;
    let t12887 = 4.0_f64 / 15.0_f64 * t2146 * t3829;
    let t12889 = 8.0_f64 / 15.0_f64 * t4763 * t3421;
    let t12890 = t2146 * t3864;
    let t12891 = 16.0_f64 / 45.0_f64 * t12890;
    let t12893 = 8.0_f64 / 15.0_f64 * t4738 * t3819;
    let t12895 = 4.0_f64 / 9.0_f64 * t2146 * t3982;
    (t12881, t12883, t12885, t12887, t12889, t12891, t12893, t12895)
}
