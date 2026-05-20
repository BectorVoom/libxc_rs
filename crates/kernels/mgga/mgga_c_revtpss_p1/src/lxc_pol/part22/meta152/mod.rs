//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta152 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1008;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1009;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1010;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1011;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1012;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1013;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1014;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1015;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1016;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta152<F: Float>(t1248: F, t482: F, t471: F, t3153: F, t1042: F, t1244: F, t3598: F, t3594: F, t1121: F, t414: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t3601 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1008::<F>(t1248);
        let (t3602, t3603) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1009::<F>(t3601, t482, t471);
        let t3604 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1010::<F>(t3153, t3603);
        let (t3605, t3606) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1011::<F>(t3602, t3604, t1042);
        let t3609 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1012::<F>(t1244, t3598);
        let t3610 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1013::<F>(t3594, t3609);
        let t3611 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1014::<F>(t3153, t471);
        let (t3612, t3613) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1015::<F>(t3602, t3611, t1042);
        let t3617 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1016::<F>(t1121, t414);
    (t3601, t3603, t3604, t3605, t3606, t3609, t3610, t3611, t3612, t3613, t3617)
}
