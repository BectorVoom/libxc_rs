//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta40 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk266;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk267;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk268;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk269;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk270;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk271;
use chunk6::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk272;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta40<F: Float>(t787: F, t789: F, t579: F, t65: F, t64: F, t159: F, t222: F, t228: F, t216: F, t136: F, t220: F, t124: F, t775: F, t212: F, t27: F, t235: F, t240: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t791, t793) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk266::<F>(t787, t789, t579, t65);
        let t794 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk267::<F>(t64, t793);
        let (t795, t797, t798, t799) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk268::<F>(t159, t794, t222, t228, t216);
        let t800 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk269::<F>(t136, t220);
        let t802 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk270::<F>(t124, t775, t800);
        let t807 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk271::<F>(t124, t212, t27, t800);
        let t808 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk272::<F>(t235, t240);
    (t791, t793, t794, t795, t797, t798, t799, t800, t802, t807, t808)
}
