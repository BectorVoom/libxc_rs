//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 834/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk834(t7415: f64, t7474: f64, t7877: f64, t7929: f64, t2684: f64, t907: f64, t2693: f64, t902: f64, t906: f64, t317: f64, t2695: f64, t956: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7931 = t7415 + t7474 + t7877 + t7929;
    let t7935 = t2684 * t907;
    let t7939 = t902 * t2693;
    let t7946 = t906 * t906;
    let t7947 = 1.0_f64 / t7946;
    let t7948 = t317 * t7947;
    let t7949 = t2695 * t956;
    (t7931, t7935, t7939, t7946, t7947, t7948, t7949)
}
