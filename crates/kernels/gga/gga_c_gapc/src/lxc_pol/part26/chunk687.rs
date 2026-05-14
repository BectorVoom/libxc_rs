//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 687/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk687<F: Float>(t1044: F, t998: F, t169: F, t1019: F, t3017: F, t5983: F, t1043: F, t1845: F, t191: F, t3018: F, t5079: F, t3016: F, t1803: F, t5017: F, t3022: F, t3028: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t8724 = t998 * t1044;
    let t8725 = t169 * t8724;
    let t8726 = t8725 * t1019;
    let t8728 = t3017 * t5983;
    let t8729 = t1043 * t8728;
    let t8731 = t1845 * t191;
    let t8732 = t8731 * t3018;
    let t8734 = t3017 * t5079;
    let t8735 = t3016 * t8734;
    let t8737 = t1803 * t191;
    let t8738 = t3017 * t5017;
    let t8739 = t8737 * t8738;
    let t8741 = t3028 * t3022;
    (t8725, t8726, t8728, t8729, t8732, t8734, t8735, t8738, t8739, t8741)
}
