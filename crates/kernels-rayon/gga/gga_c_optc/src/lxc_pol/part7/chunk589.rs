//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 589/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk589(t1200: f64, t1205: f64, t2879: f64, t2881: f64, t2886: f64, t2887: f64, t2900: f64, t485: f64, t275: f64, t176: f64, t1186: f64, t474: f64, sigma2: f64) -> (f64, f64, f64, f64) {
    let t2902 = -t1200 * t2900 - 2.0_f64 * t2881 * t1205 + t2879 * t485 + 2.0_f64 * t2886 * t2887;
    let t2903 = t2902 * t275;
    let t2905 = t176 * t2903 * sigma2;
    let t2908 = t1186 * t1186;
    let t2910 = t474 * t474;
    (t2902, t2905, t2908, t2910)
}
