//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta111 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk630;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk631;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk632;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk633;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta111<F: Float>(t2453: F, t556: F, t136: F, t561: F, t2457: F, t1426: F, t786: F, t1363: F, t2470: F, t1362: F, t1386: F, t820: F, t843: F, t241: F, t1412: F, t72: F, t245: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3906, t3907, t3908, t3910, t3914, t3915) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk630::<F>(t2453, t556, t136, t561, t2457, t1426, t786);
        let (t3920, t3922, t3930) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk631::<F>(t1363, t2470, t1362, t1386, t820, t843);
        let t3934 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk632::<F>(t1386, t241, t820);
        let (t3935, t3936) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk633::<F>(t1412, t72, t245);
    (t3906, t3907, t3908, t3910, t3914, t3915, t3920, t3922, t3930, t3934, t3935, t3936)
}
