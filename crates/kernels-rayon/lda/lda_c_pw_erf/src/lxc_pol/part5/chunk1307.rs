//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1307/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1307(t12047: f64, t12051: f64, t15971: f64, t20976: f64, t20978: f64, t20980: f64, t20981: f64, t20983: f64, t20985: f64, t20987: f64, t20988: f64, t20990: f64, t20992: f64) -> f64 {
    let t23208 = -t20976 + 4.0_f64 * t15971 - t20978 + t20980 + t20981 + t20983 - t20985 - t12047 + t12051 - t20987 - t20988 + t20990 + t20992;
    t23208
}
