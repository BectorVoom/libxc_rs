//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 842/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk842(t5200: f64, t5202: f64, t5203: f64, t5204: f64, t5213: f64, t5217: f64, t5224: f64, t5228: f64, t5233: f64, t5236: f64, t5240: f64, t5242: f64, t5246: f64, t5249: f64, t5253: f64, t5259: f64, t5263: f64) -> f64 {
    let t5867 = t5200 + t5202 + t5203 + t5204 + t5213 + t5217 - t5224 + t5228 + t5233 - t5236 + t5240 - t5242 - t5246 + t5249 + t5253 + t5259 - t5263;
    t5867
}
