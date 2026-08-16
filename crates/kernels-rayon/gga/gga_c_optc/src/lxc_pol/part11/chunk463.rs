//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 463/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk463(t2843: f64, t1065: f64, t398: f64, t393: f64, t373: f64, t376: f64, t2865: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2922 = 0.22831111111111111111e-1_f64 * t2843;
    let t2933 = t1065 * t398;
    let t2934 = 1.0_f64 / t2933;
    let t2935 = t393 * t2934;
    let t2941 = 1.0_f64 / t376 / t373;
    let t2945 = 4.0_f64 / 9.0_f64 * t2843;
    let t2953 = 0.68863333333333333333e0_f64 * t2843;
    let t2958 = 1.0_f64/f64::sqrt(t373);
    let t2963 = 0.17365833333333333333e0_f64 * t2865;
    let t2972 = t1065 * t1065;
    (t2922, t2933, t2934, t2935, t2941, t2945, t2953, t2958, t2963, t2972)
}
