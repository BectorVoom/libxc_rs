//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta20 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk157;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk158;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk159;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk160;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk161;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk162;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk163;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta20<F: Float>(t345: F, t348: F, t367: F, t375: F, t225: F, t359: F, t342: F, t198: F, t293: F, t328: F, t330: F, t336: F, t265: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t378 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk157::<F>(t345, t348, t367, t375);
        let t379 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk158::<F>(t225, t378);
        let t380 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk159::<F>(t225, t359);
        let t381 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk160::<F>(t378, t380);
        let (t384, t385) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk161::<F>(t342, t381);
        let t386 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk162::<F>(t379, t385);
        let (t389, t395, t393) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk163::<F>(t342, t386, t198, t293, t328, t330, t336, t265);
    (t378, t379, t380, t381, t384, t385, t386, t389, t395, t393)
}
