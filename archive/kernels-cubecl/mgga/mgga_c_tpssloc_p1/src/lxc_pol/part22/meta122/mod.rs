//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta122 (260520-c91 hierarchical CSE).
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
mod chunk9;
mod chunk10;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk821;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk822;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk823;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk824;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk825;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk826;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk827;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk828;
use chunk8::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk829;
use chunk9::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk830;
use chunk10::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk831;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta122<F: Float>(t1243: F, t3534: F, t3032: F, t3502: F, t3499: F, t1932: F, t3508: F, t1209: F, t475: F, t500: F, t526: F, t528: F, t118: F, t521: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t3604 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk821::<F>(t1243, t3534);
        let t3609 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk822::<F>(t3032, t3502);
        let t3610 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk823::<F>(t3499, t3609);
        let t3612 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk824::<F>(t1932, t3508);
        let t3623 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk825::<F>(t1209, t3032);
        let t3624 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk826::<F>(t3499, t3623);
        let t3625 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk827::<F>(t1932, t475);
        let (t3639, t3640) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk828::<F>(t500);
        let t3664 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk829::<F>(t526);
        let t3672 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk830::<F>(t528);
        let t3684 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk831::<F>(t118, t521);
    (t3604, t3609, t3610, t3612, t3623, t3624, t3625, t3639, t3640, t3664, t3672, t3684)
}
