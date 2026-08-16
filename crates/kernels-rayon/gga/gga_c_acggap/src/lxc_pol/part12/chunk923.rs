//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 923/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk923(t1083: f64, t31024: f64, t2095: f64, t3120: f64, t368: f64, t7380: f64, t1061: f64, t429: f64, t130: f64, t1964: f64, t2037: f64, t377: f64, t7684: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t31025 = t1083 * t31024;
    let t31026 = t2095 * t31025;
    let t31028 = t368 * t3120;
    let t31029 = t7380 * t31028;
    let t31032 = t429 * t1061;
    let t31033 = t7380 * t31032;
    let t31035 = t130 * t1964;
    let t31036 = t31035 * t2037;
    let t31038 = t377 * t7684;
    (t31025, t31026, t31028, t31029, t31032, t31033, t31035, t31036, t31038)
}
