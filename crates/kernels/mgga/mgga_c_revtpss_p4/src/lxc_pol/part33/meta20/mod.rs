//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta20 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk142;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk143;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk144;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk145;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk146;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk147;
use chunk6::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk148;
use chunk7::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk149;
use chunk8::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk150;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta20<F: Float>(t369: F, t72: F, t245: F, t125: F, t66: F, t283: F, t345: F, t348: F, t367: F, t225: F, t359: F, t342: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t370, t371) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk142::<F>(t369, t72, t245);
        let t372 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk143::<F>(t125, t66);
        let t373 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk144::<F>(t283);
        let t375 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk145::<F>(t371, t372, t373);
        let t378 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk146::<F>(t345, t348, t367, t375);
        let (t379, t380) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk147::<F>(t225, t378, t359);
        let t381 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk148::<F>(t378, t380);
        let (t384, t385) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk149::<F>(t342, t381);
        let t386 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk150::<F>(t379, t385);
    (t370, t371, t372, t373, t375, t378, t379, t380, t381, t384, t385, t386)
}
