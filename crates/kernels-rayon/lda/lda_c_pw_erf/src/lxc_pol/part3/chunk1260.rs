//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1260/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1260(t12001: f64, t12003: f64, t12008: f64, t12010: f64, t12012: f64, t12014: f64, t12016: f64, t12018: f64, t12020: f64, t12022: f64, t12024: f64, t12028: f64, t12034: f64) -> f64 {
    let t14986 = t12001 + t12003 + t12008 - t12010 + t12012 - t12014 - t12016 - t12018 + t12020 + t12022 + t12024 - t12028 + t12034;
    t14986
}
