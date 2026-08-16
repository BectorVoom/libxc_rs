//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 488/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk488(t1308: f64, t2388: f64, t571: f64, t2005: f64, t739: f64, t1326: f64) -> (f64, f64, f64, f64) {
    let t2389 = t1308 * t2388;
    let t2391 = 8.0_f64 / 45.0_f64 * t571 * t2389;
    let t2392 = t2005 * t739;
    let t2393 = t1326 * t2392;
    (t2389, t2391, t2392, t2393)
}
