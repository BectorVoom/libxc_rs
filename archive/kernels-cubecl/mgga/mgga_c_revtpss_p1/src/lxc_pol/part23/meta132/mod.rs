//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta132 (260520-c91 hierarchical CSE).
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
mod chunk11;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk856;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk857;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk858;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk859;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk860;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk861;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk862;
use chunk7::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk863;
use chunk8::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk864;
use chunk9::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk865;
use chunk10::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk866;
use chunk11::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk867;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta132<F: Float>(t1234: F, t1260: F, t1209: F, t1284: F, t3624: F, t482: F, t66: F, t828: F, t1269: F, t460: F, t1275: F, t493: F, t225: F, t1204: F, t487: F, t473: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t3711 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk856::<F>(t1234, t1260);
        let t3717 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk857::<F>(t1209, t1284);
        let t3718 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk858::<F>(t3624, t3717);
        let t3719 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk859::<F>(t482, t66);
        let t3720 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk860::<F>(t3719, t828);
        let t3732 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk861::<F>(t1269, t460);
        let t3736 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk862::<F>(t1275, t493);
        let t3737 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk863::<F>(t225, t3736);
        let t3746 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk864::<F>(t1204, t1284);
        let t3754 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk865::<F>(t1284, t487);
        let t3755 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk866::<F>(t1209, t3754);
        let t3759 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk867::<F>(t1269, t473);
    (t3711, t3717, t3718, t3719, t3720, t3732, t3736, t3737, t3746, t3754, t3755, t3759)
}
