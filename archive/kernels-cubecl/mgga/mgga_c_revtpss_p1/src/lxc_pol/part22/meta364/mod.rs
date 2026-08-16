//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta364 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1892;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1893;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1894;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1895;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1896;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1897;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1898;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta364<F: Float>(t1248: F, t3153: F, t3588: F, t5464: F, t3566: F, t3754: F, t1269: F, t1284: F, t1209: F, t1204: F, t3781: F, t5462: F, t5477: F, t3634: F, t828: F, t3630: F, t3625: F, t3624: F, t3746: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t12712, t12713, t12717) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1892::<F>(t1248, t3153, t3588, t5464, t3566, t3754);
        let (t12722, t12723) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1893::<F>(t1269, t1284, t1209);
        let t12744 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1894::<F>(t1204, t3781);
        let t12751 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1895::<F>(t1209, t5462);
        let t12756 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1896::<F>(t1209, t5477);
        let t12772 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1897::<F>(t3634, t828);
        let (t12773, t12774, t12784) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1898::<F>(t12772, t3630, t3625, t3624, t3746);
    (t12712, t12713, t12717, t12722, t12723, t12744, t12751, t12756, t12772, t12773, t12774, t12784)
}
