//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 188/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk188(t176: f64, t511: f64, t166: f64, t161: f64, t175: f64) -> (f64, f64, f64, f64, f64) {
    let t512 = t511 * t176;
    let t513 = t166 * t512;
    let t515 = t161 * t513 / 30.0_f64;
    let t516 = t175 * t175;
    let t517 = 1.0_f64 / t516;
    (t512, t513, t515, t516, t517)
}
