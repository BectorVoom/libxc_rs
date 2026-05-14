//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1081/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1081<F: Float>(t2667: F, t2730: F, t312: F, t508: F, t2670: F, t2678: F, t2679: F, t2668: F, t2674: F, t2606: F, t2669: F, t3835: F, t7354: F, t8143: F, t2269: F, t2672: F) -> (F, F, F, F, F, F, F) {
    let t24510 = t2730 * t2667;
    let t24513 = t508 * t312;
    let t24514 = t24513 * t2670;
    let t24516 = t2678 * t24514 * t2679;
    let t24519 = t2668 * t24514 * t2674;
    let t24521 = t2606 * t2606;
    let t24522 = t2669 * t24521;
    let t24530 = t3835 * t8143 * t7354;
    let t24535 = t2672 * t2269;
    (t24510, t24516, t24519, t24521, t24522, t24530, t24535)
}
