//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta326 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1613;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1614;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1615;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1616;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1617;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1618;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1619;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta326<F: Float>(t11142: F, t11145: F, t128: F, t2851: F, t45: F, t10356: F, t2850: F, t2258: F, t2852: F, t606: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t11146, t11147) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1613::<F>(t11142, t11145, t128);
        let (t11149, t11150) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1614::<F>(t2851, t45);
        let t11151 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1615::<F>(t10356, t11150);
        let (t11152, t11153) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1616::<F>(t11151, t2850, t128);
        let t11156 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1617::<F>(t2258, t2852, t606);
        let (t11157, t11158) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1618::<F>(t11156, t2850, t128);
        let t11160 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1619::<F>(t10356, t2852);
    (t11146, t11147, t11149, t11150, t11151, t11152, t11153, t11156, t11157, t11158, t11160)
}
