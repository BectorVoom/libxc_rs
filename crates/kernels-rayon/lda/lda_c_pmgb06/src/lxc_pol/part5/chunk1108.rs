//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1108/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1108(t20318: f64, t2489: f64, t5194: f64, t20293: f64, t20296: f64, t20299: f64, t20302: f64, t20305: f64, t20308: f64, t20311: f64, t20314: f64, t20317: f64) -> (f64, f64, f64) {
    let t20319 = 4.0_f64 / 45.0_f64 * t20318;
    let t20320 = t5194 * t2489;
    let t20321 = 4.0_f64 / 45.0_f64 * t20320;
    let t20322 = t20293 - t20296 - t20299 + t20302 + t20305 - t20308 - t20311 + t20314 - t20317 - t20319 - t20321;
    (t20319, t20321, t20322)
}
