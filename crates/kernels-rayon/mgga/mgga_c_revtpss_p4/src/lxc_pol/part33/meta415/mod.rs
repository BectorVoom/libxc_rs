//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta415 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1481;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1482;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1483;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta415(t18498: f64, t2477: f64, t828: f64, t5984: f64, t775: f64, t800: f64, t5988: f64, t1548: f64, t4343: f64, t10811: f64, t6037: f64, t18444: f64, t4364: f64, t4366: f64, t10846: f64, t10885: f64, t10888: f64, t10891: f64, t10900: f64, t18491: f64, t18495: f64, t2730: f64, t4362: f64, t851: f64, t10871: f64, t836: f64, t18426: f64, t221: f64, t2485: f64, t5978: f64, t2484: f64, t10552: f64, t10554: f64, t14317: f64, t18261: f64, t18262: f64, t18265: f64, t18267: f64, t18300: f64, t18301: f64, t18308: f64, t9278: f64, t9308: f64, t9316: f64, t9329: f64, t9333: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18500, t18507, t18511, t18515, t18518, t18521) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1481(t18498, t2477, t828, t5984, t775, t800, t5988, t1548, t4343, t10811, t6037, t18444, t4364, t4366);
        let t18524 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1482(t10846, t10885, t10888, t10891, t10900, t18491, t18495, t18500, t18507, t18511, t18515, t18518, t18521, t2730, t4362, t851);
        let (t18525, t18527, t18531, t18532, t18534) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1483(t10871, t836, t18426, t4364, t221, t2485, t5978, t2484, t10552, t10554, t14317, t18261, t18262, t18265, t18267, t18300, t18301, t18308, t9278, t9308, t9316, t9329, t9333);
    (t18500, t18521, t18524, t18525, t18527, t18531, t18532, t18534)
}
