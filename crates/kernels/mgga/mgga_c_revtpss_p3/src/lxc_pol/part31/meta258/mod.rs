//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta258 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1141;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1142;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1143;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1144;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1145;
use chunk5::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1146;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta258<F: Float>(t234: F, t243: F, t7028: F, t807: F, t1945: F, t786: F, t817: F, t64: F, t822: F, t239: F, t820: F, t839: F, t1946: F, t846: F, t233: F, t857: F, t7024: F, t7026: F, t225: F, t1949: F, t213: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t7030, t7032, t7033, t7035, t7036) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1141::<F>(t234, t243, t7028, t807, t1945, t786, t817, t64, t822);
        let t7038 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1142::<F>(t239, t7036, t820);
        let (t7039, t7042, t7043) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1143::<F>(t7038, t839, t1946, t846, t233, t64);
        let t7045 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1144::<F>(t239, t7043, t820);
        let t7048 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1145::<F>(t7045, t857, t7024, t7026, t7032, t7035, t7039, t7042);
        let (t7049, t7053) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1146::<F>(t225, t7048, t1949, t213);
    (t7030, t7032, t7033, t7035, t7036, t7038, t7042, t7043, t7045, t7048, t7049, t7053)
}
