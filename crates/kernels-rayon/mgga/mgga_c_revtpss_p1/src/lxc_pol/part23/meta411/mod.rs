//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta411 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1789;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1790;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1791;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta411(t45: f64, t57: f64, t5819: f64, t633: f64, t5825: f64, t80: f64, t18281: f64, t4186: f64, t4328: f64, t606: f64, t766: f64, t637: f64, t83: f64, t4335: f64, t770: f64, zeta_threshold: f64, t124: f64, t800: f64, t828: f64, t855: f64, t221: f64, t2675: f64, t5962: f64, t2674: f64, t10756: f64, t10758: f64, t10762: f64, t14836: f64, t14837: f64, t14839: f64, t14846: f64, t14850: f64, t14859: f64, t14864: f64, t799: f64, t851: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18367, t18378, t18379, t18390) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1789(t45, t57, t5819, t633, t5825, t80, t18281, t4186, t4328, t606, t766, t637, t83, t4335, t770, zeta_threshold);
        let t18392 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1790(t18378, t18390);
        let (t18393, t18394, t18398, t18402, t18403, t18405) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1791(t124, t18392, t800, t828, t855, t221, t2675, t5962, t2674, t10756, t10758, t10762, t14836, t14837, t14839, t14846, t14850, t14859, t14864, t799, t851);
    (t18367, t18379, t18392, t18393, t18394, t18398, t18402, t18403, t18405)
}
