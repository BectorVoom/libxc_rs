//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta528 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2311;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2312;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2313;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta528<F: Float>(t17240: F, t5052: F, t1222: F, t16738: F, t5308: F, t16742: F, t16733: F, t16771: F, t247: F, t3719: F, t3636: F, t5391: F, t5381: F, t1260: F, t12966: F, t16775: F, t1261: F, t17232: F, t17237: F, t5384: F, t5386: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t17241, t17243, t17244, t17247, t17250, t17254, t17258) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2311::<F>(t17240, t5052, t1222, t16738, t5308, t16742, t16733, t16771, t247, t3719, t3636, t5391);
        let (t17260, t17261) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2312::<F>(t3636, t5381, t1260, t12966);
        let (t17265, t17268) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2313::<F>(t16775, t247, t3719, t1222, t1261, t17232, t17237, t17243, t17244, t17247, t17250, t17254, t17258, t17260, t17261, t5384, t5386);
    (t17241, t17243, t17244, t17247, t17250, t17254, t17258, t17260, t17261, t17265, t17268)
}
