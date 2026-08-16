//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 790/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk790(t494: f64, t806: f64, t542: f64, t5289: f64, t1325: f64, t1392: f64, t789: f64, t3806: f64, t519: f64, t1326: f64, t4628: f64, t2022: f64, t3863: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5290 = t806 * t494;
    let t5291 = t5290 * t542;
    let t5292 = t5289 * t5291;
    let t5294 = 16.0_f64 / 15.0_f64 * t1325 * t5292;
    let t5295 = t789 * t1392;
    let t5296 = t3806 * t5295;
    let t5298 = 8.0_f64 / 45.0_f64 * t519 * t5296;
    let t5299 = t1326 * t4628;
    let t5301 = 8.0_f64 / 15.0_f64 * t519 * t5299;
    let t5302 = t3863 * t2022;
    (t5290, t5291, t5292, t5294, t5295, t5296, t5298, t5299, t5301, t5302)
}
