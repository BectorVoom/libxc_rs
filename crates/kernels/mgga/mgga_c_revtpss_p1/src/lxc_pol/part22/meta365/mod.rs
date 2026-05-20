//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta365 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1899;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1900;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1901;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1902;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta365<F: Float>(t3618: F, t828: F, t1260: F, t3650: F, t3588: F, t73: F, t1209: F, t3781: F, t5330: F, t3153: F, t3601: F, t1284: F, t3555: F, t3624: F) -> (F, F, F, F, F, F, F, F) {
        let t12787 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1899::<F>(t3618, t828);
        let t12800 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1900::<F>(t1260, t3650);
        let (t12803, t12808, t12809) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1901::<F>(t3588, t73, t1209, t3781, t5330);
        let (t12810, t12831, t12832) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1902::<F>(t3153, t3601, t1284, t3555, t3624);
    (t12787, t12800, t12803, t12808, t12809, t12810, t12831, t12832)
}
