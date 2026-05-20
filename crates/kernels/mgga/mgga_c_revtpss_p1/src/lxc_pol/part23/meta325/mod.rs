//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta325 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1618;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1619;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1620;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1621;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta325<F: Float>(t1882: F, t4003: F, t3957: F, t5690: F, t1873: F, t9741: F, t5651: F, t808: F, t9736: F, t241: F, t820: F, t9991: F, t5697: F, t9962: F, t5701: F, t5608: F, t5675: F, t9934: F, t2661: F, t2482: F, t4000: F, t814: F, t136: F, t550: F, t220: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t13790 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1618::<F>(t1882, t4003);
        let (t13797, t13798, t13800, t13801, t13804, t13810) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1619::<F>(t3957, t5690, t1873, t9741, t5651, t808, t9736, t241, t820, t9991, t5697, t9962);
        let (t13813, t13830, t13832, t13845) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1620::<F>(t5701, t9962, t5608, t5675, t9934, t2661, t2482, t4000, t814);
        let t13847 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1621::<F>(t136, t550, t220);
    (t13790, t13797, t13798, t13800, t13801, t13804, t13810, t13813, t13830, t13832, t13845, t13847)
}
