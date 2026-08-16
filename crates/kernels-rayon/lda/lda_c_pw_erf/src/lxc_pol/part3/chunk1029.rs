//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1029/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1029(t12034: f64, t12039: f64, t12040: f64, t12041: f64, t12042: f64, t12043: f64, t12045: f64, t12047: f64, t12049: f64, t12051: f64, t12053: f64, t12055: f64, t12059: f64) -> f64 {
    let t12060 = t12034 + t12039 + t12040 + t12041 + t12042 + t12043 - t12045 - t12047 - t12049 + t12051 - t12053 - t12055 + t12059;
    t12060
}
