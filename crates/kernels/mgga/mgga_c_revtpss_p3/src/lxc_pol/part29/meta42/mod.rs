//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta42 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk269;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk270;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk271;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk272;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk273;
use chunk5::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk274;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta42<F: Float>(t234: F, t243: F, t808: F, t807: F, t236: F, t786: F, t240: F, t27: F, t124: F, t800: F, t213: F, t225: F, t232: F, t235: F, t239: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t810, t812, t813, t814) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk269::<F>(t234, t243, t808, t807, t236, t786, t240, t27);
        let (t815, t816) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk270::<F>(t243, t814, t124, t800);
        let (t817, t819, t820) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk271::<F>(t815, t816, t813, t213, t225);
        let (t821, t822) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk272::<F>(t232);
        let t823 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk273::<F>(t235, t822);
        let t825 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk274::<F>(t239, t820, t823);
    (t810, t812, t813, t814, t816, t817, t819, t820, t821, t822, t823, t825)
}
