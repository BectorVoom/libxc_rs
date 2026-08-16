//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 761/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk761<F: Float>(t1803: F, t191: F, t3017: F, t5017: F, t3022: F, t3028: F, t1033: F, t5486: F, t169: F, t474: F, t619: F, t116: F, t5463: F) -> (F, F, F, F, F) {
    let t8737 = t1803 * t191;
    let t8738 = t3017 * t5017;
    let t8739 = t8737 * t8738;
    let t8741 = t3028 * t3022;
    let t8743 = t5486 * t1033;
    let t8744 = t169 * t8743;
    let t8745 = t474 * t619;
    let t8746 = t8744 * t8745;
    let t8748 = t116 * t5463;
    (t8738, t8739, t8741, t8746, t8748)
}
