//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta263 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1460;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1461;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1462;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1463;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta263<F: Float>(t1416: F, t9779: F, t124: F, t212: F, t2237: F, t800: F, t1376: F, t123: F, t125: F, t2452: F, t9720: F, t235: F, t4086: F, t2453: F, t240: F, t2712: F, t3994: F, t2713: F, t3951: F, t3964: F, t785: F, t9731: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t9780, t9784) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1460::<F>(t1416, t9779, t124, t212, t2237, t800);
        let (t9786, t9789, t9791, t9792, t9793) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1461::<F>(t1376, t9784, t123, t125, t2452, t9720, t235, t4086, t2453);
        let t9794 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1462::<F>(t240, t2712);
        let (t9796, t9799, t9801) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1463::<F>(t3994, t9794, t9793, t2713, t3951, t3964, t785, t9731);
    (t9780, t9784, t9786, t9789, t9791, t9792, t9793, t9794, t9796, t9799, t9801)
}
