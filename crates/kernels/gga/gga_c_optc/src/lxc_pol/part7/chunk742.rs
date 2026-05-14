//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 742/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk742<F: Float>(t2667: F, t946: F, t312: F, t9: F, t2670: F, t2674: F, t2668: F, t2679: F, t2678: F, t2574: F, t858: F, t2579: F, t854: F, t116: F, t7328: F, t286: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7430 = t946 * t2667;
    let t7433 = t9 * t312;
    let t7434 = t7433 * t2670;
    let t7435 = t7434 * t2674;
    let t7436 = t2668 * t7435;
    let t7438 = t7434 * t2679;
    let t7439 = t2678 * t7438;
    let t7441 = t2574 * t858;
    let t7443 = t854 * t2579;
    let t7445 = t116 * t7328;
    let t7447 = 5.0 / 1296.0 * t286 * t7445;
    (t7430, t7433, t7435, t7436, t7438, t7439, t7441, t7443, t7445, t7447)
}
