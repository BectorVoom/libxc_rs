//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 413/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk413(t1773: f64, t1782: f64, t720: f64, t723: f64, t182: f64, t722: f64, t179: f64, t727: f64) -> (f64, f64, f64, f64, f64) {
    let t2211 = 0.25851111111111111111e1_f64 * t1773 + 0.20525e-2_f64 * t1782;
    let t2213 = t720 * t723;
    let t2217 = 1.0_f64 / t722 / t182;
    let t2218 = t179 * t2217;
    let t2219 = t727 * t727;
    (t2211, t2213, t2217, t2218, t2219)
}
