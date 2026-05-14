//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 975/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk975<F: Float>(t16845: F, t18196: F, t47675: F, t47676: F, t47677: F, t47678: F, t47679: F, t47683: F, t47684: F, t47685: F, t47687: F, t47691: F, t12544: F, t30876: F, t12583: F, t1620: F, t1621: F, t25081: F) -> (F, F, F) {
    let t47692 = t47675 + t18196 + t47676 + t47677 - t47678 + t47679 - t47683 - t16845 - t47684 + t47685 + t47687 - t47691;
    let t47695 = 16.0 / 5.0 * t30876 * t12544;
    let t47699 = 32.0 / 5.0 * t1620 * t1621 * t25081 * t12583;
    (t47692, t47695, t47699)
}
