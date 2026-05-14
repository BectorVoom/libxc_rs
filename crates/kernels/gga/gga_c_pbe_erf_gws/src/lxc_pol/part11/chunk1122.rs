//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1122/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1122<F: Float>(t49625: F, t49634: F, t49641: F, t49643: F, t49650: F, t49652: F, t49658: F, t49660: F, t49661: F, t49663: F, t49664: F, t49667: F, t49671: F, t49672: F, t49673: F, t49681: F, t49683: F, t49687: F, t49696: F, t49717: F, t49722: F, t49729: F) -> (F, F) {
    let t50572 = t49625 - t49634 - t49641 - t49643 + t49650 + t49652 + t49658 - t49660 + t49661 - t49663 - t49664;
    let t50574 = -t49667 - t49671 - t49672 + t49673 - t49681 - t49683 + t49687 + t49696 - t49717 + t49722 - t49729;
    (t50572, t50574)
}
