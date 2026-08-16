//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 573/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk573(t1022: f64, t1030: f64, t385: f64, t907: f64, t935: f64, t333: f64, t904: f64, t335: f64, t913: f64, t905: f64, t334: f64, t317: f64, t902: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3111 = t1022 * t1030;
    let t3112 = t3111 * t385;
    let t3115 = t935 * t907;
    let t3117 = t904 * t3115 * t333;
    let t3118 = 48.24547296645331_f64 * t3117;
    let t3120 = t913 * t335 * t935;
    let t3121 = 6.0_f64 * t3120;
    let t3122 = t905 * t333;
    let t3123 = t3122 * t334;
    let t3124 = t904 * t3123;
    let t3125 = 6.0_f64 * t3124;
    let t3127 = 1.0_f64 / t902 / t317;
    (t3111, t3112, t3115, t3117, t3118, t3120, t3121, t3122, t3123, t3124, t3125, t3127)
}
