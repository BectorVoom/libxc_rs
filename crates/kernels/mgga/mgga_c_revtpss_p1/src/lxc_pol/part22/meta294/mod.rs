//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta294 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1716;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1717;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1718;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1719;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta294<F: Float>(t1386: F, t2482: F, t814: F, t136: F, t1412: F, t220: F, t124: F, t1398: F, t3938: F, t1353: F, t4003: F, t4056: F, t2735: F, t4086: F) -> (F, F, F, F, F, F, F, F) {
        let t9816 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1716::<F>(t1386, t2482, t814);
        let (t9817, t9818) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1717::<F>(t136, t1412, t220);
        let (t9821, t9822, t9835, t9840) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1718::<F>(t124, t1398, t3938, t9818, t9816, t1353, t4003, t4056);
        let t9845 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1719::<F>(t2735, t4086);
    (t9816, t9817, t9818, t9821, t9822, t9835, t9840, t9845)
}
