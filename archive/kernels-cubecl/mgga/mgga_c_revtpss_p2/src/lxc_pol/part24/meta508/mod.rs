//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta508 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1519;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1520;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1521;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1522;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1523;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta508<F: Float>(t23489: F, t689: F, t23482: F, t23486: F, t23500: F, t23504: F) -> (F, F, F, F, F) {
        let t77505 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1519::<F>(t23489, t689);
        let t77507 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1520::<F>(t23482, t689);
        let t77509 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1521::<F>(t23486, t689);
        let t77559 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1522::<F>(t23500, t689);
        let t77561 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1523::<F>(t23504, t689);
    (t77505, t77507, t77509, t77559, t77561)
}
