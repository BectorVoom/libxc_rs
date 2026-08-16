//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta411 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1469;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1470;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1471;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1472;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1473;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta411<F: Float>(t14494: F, t6035: F, t14791: F, t2703: F, t5985: F, t10905: F, t5989: F, t10678: F, t10687: F, t10692: F, t14736: F, t14744: F, t14759: F, t14761: F, t14765: F, t14777: F, t2745: F, t5962: F, t854: F, t236: F, t807: F, t2476: F, t5966: F, t10717: F, t10719: F, t10723: F, t10746: F, t10749: F, t14780: F, t14783: F, t14817: F, t14820: F, t14823: F, t45: F, t57: F, t5819: F, t633: F, t5825: F, t80: F, t18281: F, t4186: F, t4328: F, t606: F, t766: F, t637: F, t83: F, t4335: F, t770: F, zeta_threshold: F, t124: F, t800: F, t828: F, t855: F, t221: F, t2675: F, t2674: F, t10756: F, t10758: F, t10762: F, t14836: F, t14837: F, t14839: F, t14846: F, t14850: F, t14859: F, t14864: F, t799: F, t851: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t18334, t18343) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1469::<F>(t14494, t6035, t14791, t2703, t5985, t10905, t5989, t10678, t10687, t10692, t14736, t14744, t14759, t14761, t14765, t14777, t2745);
        let (t18348, t18352, t18361) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1470::<F>(t5962, t854, t236, t807, t2476, t5966, t10717, t10719, t10723, t10746, t10749, t14780, t14783, t14817, t14820, t14823);
        let (t18378, t18390) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1471::<F>(t45, t57, t5819, t633, t5825, t80, t18281, t4186, t4328, t606, t766, t637, t83, t4335, t770, zeta_threshold);
        let t18392 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1472::<F>(t18378, t18390);
        let (t18394, t18398, t18402, t18405) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1473::<F>(t124, t18392, t800, t828, t855, t221, t2675, t5962, t2674, t10756, t10758, t10762, t14836, t14837, t14839, t14846, t14850, t14859, t14864, t799, t851);
    (t18334, t18343, t18348, t18352, t18361, t18392, t18394, t18398, t18402, t18405)
}
