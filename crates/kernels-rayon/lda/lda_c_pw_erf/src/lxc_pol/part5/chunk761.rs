//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 761/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk761(t525: f64, t6988: f64, t2478: f64, t581: f64, t593: f64, t1466: f64, t1318: f64, t6941: f64, t6943: f64, t6948: f64, t6950: f64, t6952: f64, t6956: f64, t6960: f64, t6962: f64, t6967: f64, t6972: f64, t6976: f64, t6978: f64, t6983: f64, t6985: f64, t6987: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6990 = 8.0_f64 / 45.0_f64 * t6988 * t525;
    let t6991 = t581 * t2478;
    let t6992 = t6991 * t593;
    let t6993 = t1466 * t6992;
    let t6995 = 4.0_f64 / 15.0_f64 * t1318 * t6993;
    let t6996 = t6941 + t6943 + t6948 - t6950 - t6952 - t6956 + t6960 + t6962 + t6967 - t6972 + t6976 - t6978 - t6983 - t6985 + t6987 + t6990 - t6995;
    (t6990, t6991, t6992, t6993, t6995, t6996)
}
