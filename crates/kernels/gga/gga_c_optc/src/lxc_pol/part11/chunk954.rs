//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 954/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk954<F: Float>(t312: F, t508: F, t2269: F, t2672: F, t116: F, t23533: F, t286: F, t2663: F, t140: F, t305: F) -> (F, F, F, F, F) {
    let t24513 = t508 * t312;
    let t24535 = t2672 * t2269;
    let t24546 = 5.0 / 486.0 * t286 * t116 * t23533;
    let t24562 = t2663 * t2663;
    let t24563 = 1.0 / t24562;
    let t24565 = t305 * t24563 * t140;
    (t24513, t24535, t24546, t24563, t24565)
}
