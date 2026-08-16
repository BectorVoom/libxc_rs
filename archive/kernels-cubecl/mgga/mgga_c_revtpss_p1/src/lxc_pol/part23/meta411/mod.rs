//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta411 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1789;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1790;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1791;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta411<F: Float>(t45: F, t57: F, t5819: F, t633: F, t5825: F, t80: F, t18281: F, t4186: F, t4328: F, t606: F, t766: F, t637: F, t83: F, t4335: F, t770: F, zeta_threshold: F, t124: F, t800: F, t828: F, t855: F, t221: F, t2675: F, t5962: F, t2674: F, t10756: F, t10758: F, t10762: F, t14836: F, t14837: F, t14839: F, t14846: F, t14850: F, t14859: F, t14864: F, t799: F, t851: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t18367, t18378, t18379, t18390) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1789::<F>(t45, t57, t5819, t633, t5825, t80, t18281, t4186, t4328, t606, t766, t637, t83, t4335, t770, zeta_threshold);
        let t18392 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1790::<F>(t18378, t18390);
        let (t18393, t18394, t18398, t18402, t18403, t18405) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1791::<F>(t124, t18392, t800, t828, t855, t221, t2675, t5962, t2674, t10756, t10758, t10762, t14836, t14837, t14839, t14846, t14850, t14859, t14864, t799, t851);
    (t18367, t18379, t18392, t18393, t18394, t18398, t18402, t18403, t18405)
}
