//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 792/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk792(t6941: f64, t6943: f64, t6948: f64, t6950: f64, t6952: f64, t6956: f64, t6960: f64, t6962: f64, t6967: f64, t6972: f64, t6976: f64, t6978: f64, t6983: f64, t6985: f64, t6987: f64, t6990: f64) -> f64 {
    let t7277 = t6941 + t6943 + t6948 - t6950 - t6952 - t6956 + t6960 + t6962 + t6967 - t6972 + t6976 - t6978 - t6983 - t6985 + t6987 + t6990;
    t7277
}
