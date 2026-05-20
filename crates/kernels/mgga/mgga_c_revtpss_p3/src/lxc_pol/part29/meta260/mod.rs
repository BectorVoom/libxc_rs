//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta260 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1064;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1065;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1066;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1067;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta260<F: Float>(t665: F, t6998: F, t30: F, t775: F, t159: F, t793: F, t218: F, t816: F, t1941: F, t228: F, t802: F, t240: F, t64: F, t234: F, t243: F, t807: F, t1945: F, t786: F, t817: F, t822: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t6999, t7010, t7021) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1064::<F>(t665, t6998, t30, t775, t159, t793);
        let (t7023, t7025) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1065::<F>(t218, t7021, t816, t1941, t228);
        let (t7026, t7028) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1066::<F>(t7025, t802, t240, t64);
        let (t7030, t7031, t7033, t7034, t7036) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1067::<F>(t234, t243, t7028, t807, t1945, t786, t817, t64, t822);
    (t6999, t7010, t7021, t7023, t7025, t7026, t7028, t7030, t7031, t7033, t7034, t7036)
}
