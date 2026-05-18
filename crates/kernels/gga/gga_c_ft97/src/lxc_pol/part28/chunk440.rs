//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 440/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk440<F: Float>(t143: F, t160: F, t6685: F, t1053: F, t5935: F, t144: F, t5942: F, t925: F, t2210: F, t1017: F, t1384: F) -> (F, F, F, F, F) {
    let t6687 = t143 * t6685 * t160;
    let t6691 = t5935 * t1053;
    let t6692 = t144 * t6691;
    let t6695 = t5942 * t925;
    let t6696 = t2210 * t6695;
    let t6699 = t1384 * t1017;
    (t6687, t6692, t6695, t6696, t6699)
}
