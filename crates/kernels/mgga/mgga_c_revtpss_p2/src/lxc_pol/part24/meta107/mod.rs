//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta107 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk612;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk613;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk614;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk615;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk616;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk617;
use chunk6::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk618;
use chunk7::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk619;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta107<F: Float>(t3362: F, t3698: F, t1234: F, t1260: F, t1209: F, t1284: F, t3624: F, t482: F, t66: F, t828: F, t1275: F, t493: F, t225: F, t487: F, t3140: F, t3596: F, t460: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3699, t3711) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk612::<F>(t3362, t3698, t1234, t1260);
        let t3717 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk613::<F>(t1209, t1284);
        let t3718 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk614::<F>(t3624, t3717);
        let (t3719, t3720) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk615::<F>(t482, t66, t828);
        let (t3736, t3737) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk616::<F>(t1275, t493, t225);
        let t3754 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk617::<F>(t1284, t487);
        let (t3755, t3766) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk618::<F>(t1209, t3754, t3140, t3596);
        let t3767 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk619::<F>(t3766, t460);
    (t3699, t3711, t3717, t3718, t3719, t3720, t3736, t3737, t3754, t3755, t3766, t3767)
}
