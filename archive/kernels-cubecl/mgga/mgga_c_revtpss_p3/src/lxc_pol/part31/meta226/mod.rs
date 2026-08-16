//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta226 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1008;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1009;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1010;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1011;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1012;
use chunk5::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1013;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta226<F: Float>(t225: F, t6005: F, t2638: F, t5966: F, t5962: F, t832: F, t1553: F, t1555: F, t227: F, t229: F, t231: F, t827: F, t828: F, t2723: F, t5977: F, t855: F, t1544: F, t4365: F, t2747: F, t2702: F, t2716: F, t2721: F, t2739: F, t2745: F, t4350: F, t4355: F, t4357: F, t4431: F, t825: F, t851: F, t2672: F, t2686: F, t2691: F, t2730: F, t4359: F, t4373: F, t4455: F, t5980: F, t5985: F, t5989: F, t5993: F, t799: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t6006, t6010, t6013, t6016) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1008::<F>(t225, t6005, t2638, t5966, t5962, t832, t1553, t1555, t227, t229);
        let t6017 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1009::<F>(t231, t6016);
        let (t6019, t6022) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1010::<F>(t6017, t827, t828, t2723, t5977);
        let (t6024, t6030, t6035) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1011::<F>(t6022, t827, t828, t5962, t855, t1544, t231);
        let (t6037, t6040) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1012::<F>(t4365, t6035, t2747, t2702, t2716, t2721, t2739, t2745, t4350, t4355, t4357, t4431, t6019, t6024, t6030, t825, t851);
        let t6041 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1013::<F>(t2672, t2686, t2691, t2730, t4359, t4373, t4455, t5980, t5985, t5989, t5993, t6040, t799, t825, t851);
    (t6006, t6010, t6013, t6016, t6017, t6019, t6022, t6024, t6030, t6035, t6037, t6041)
}
