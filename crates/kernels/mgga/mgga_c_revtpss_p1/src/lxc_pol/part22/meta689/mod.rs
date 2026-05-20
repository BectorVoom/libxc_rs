//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta689 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2686;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2687;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2688;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta689<F: Float>(t1398: F, t9994: F, t550: F, t6843: F, t543: F, t3992: F, t2661: F, t6861: F, t4003: F, t9934: F, t3989: F, t6856: F, t13762: F, t13763: F, t13765: F, t13772: F, t13778: F, t9711: F, t9712: F, t9725: F, t9729: F) -> (F, F, F, F, F, F, F, F) {
        let t22016 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2686::<F>(t1398, t9994);
        let (t22020, t22021, t22022, t22023, t22025) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2687::<F>(t550, t6843, t543, t3992, t2661, t6861);
        let (t22026, t22027, t22035) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2688::<F>(t22025, t4003, t9934, t2661, t3989, t6856, t13762, t13763, t13765, t13772, t13778, t22023, t9711, t9712, t9725, t9729);
    (t22016, t22020, t22021, t22022, t22025, t22026, t22027, t22035)
}
