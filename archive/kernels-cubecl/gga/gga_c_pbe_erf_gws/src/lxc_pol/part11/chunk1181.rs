//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1181/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1181<F: Float>(t47778: F, t47780: F, t47782: F, t47784: F, t47786: F, t47790: F, t47793: F, t47794: F, t47795: F, t47800: F, t47801: F, t47802: F, t47805: F, t47806: F, t47809: F, t47810: F, t47811: F, t47812: F, t47814: F, t47818: F, t47820: F, t47822: F) -> (F, F) {
    let t48646 = -t47778 - t47780 + t47782 - t47784 - t47786 - t47790 + t47793 + t47794 - t47795 + t47800 + t47801;
    let t48648 = t47802 - t47805 + t47806 + t47809 + t47810 - t47811 - t47812 + t47814 + t47818 + t47820 + t47822;
    (t48646, t48648)
}
