//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1305/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1305(t11947: f64, t11955: f64, t20921: f64, t20923: f64, t20925: f64, t20927: f64, t20929: f64, t20931: f64, t20932: f64, t20933: f64, t20934: f64, t20935: f64, t20939: f64) -> f64 {
    let t23202 = -t11947 + t20921 - t20923 - t20925 + t20927 + t20929 + t20931 - t11955 + t20932 - t20933 + t20934 - t20935 + t20939;
    t23202
}
