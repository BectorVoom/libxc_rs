//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1179/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1179<F: Float>(t16845: F, t47675: F, t47676: F, t47677: F, t47678: F, t47679: F, t47683: F, t47684: F, t47685: F, t47687: F, t47691: F, t16876: F, t47695: F, t47699: F, t47701: F, t47706: F, t47707: F, t47711: F, t47715: F, t47719: F, t47723: F, t47724: F) -> (F, F) {
    let t48638 = t47675 + t47676 + t47677 - t47678 + t47679 - t47683 - t16845 - t47684 + t47685 + t47687 - t47691;
    let t48640 = -t47695 - t47699 + t47701 + t47706 - t47707 + t47711 + t47715 - t47719 + t47723 - t16876 + t47724;
    (t48638, t48640)
}
