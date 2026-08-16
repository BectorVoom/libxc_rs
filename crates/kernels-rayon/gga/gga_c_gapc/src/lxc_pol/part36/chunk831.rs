//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 831/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk831(t9932: f64, t9934: f64, t3434: f64, t949: f64, t2749: f64, t3348: f64, t3322: f64, t9414: f64, t3330: f64, t9418: f64, t3418: f64, t7522: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9935 = t9932 * t9934;
    let t9937 = t3434 * t949;
    let t9939 = t3348 * t2749;
    let t9941 = t9414 * t3322;
    let t9944 = t9418 * t3330;
    let t9946 = t3418 * t7522;
    (t9935, t9937, t9939, t9941, t9944, t9946)
}
