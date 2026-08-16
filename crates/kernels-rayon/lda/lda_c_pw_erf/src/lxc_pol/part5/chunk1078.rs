//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1078/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1078(t169: f64, t632: f64, t7868: f64, t11323: f64, t19972: f64, t19973: f64, t19976: f64, t19977: f64, t19978: f64, t19979: f64, t19980: f64, t19981: f64, t19982: f64, t19983: f64, t19984: f64, t19985: f64, t8168: f64, t8177: f64, t8188: f64) -> (f64, f64) {
    let t20185 = t169 * t7868 * t632;
    let t20188 = t19972 - t19973 - t19976 - t8168 - t8177 - t19977 - t19978 - t19979 + t19980 + t19981 + t11323 - t19982 - t19983 - t19984 + t19985 - t8188;
    (t20185, t20188)
}
