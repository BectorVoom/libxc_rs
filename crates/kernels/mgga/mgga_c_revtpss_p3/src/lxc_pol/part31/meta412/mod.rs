//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta412 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1474;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1475;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1476;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta412<F: Float>(t18498: F, t2477: F, t828: F, t5984: F, t775: F, t800: F, t5988: F, t1548: F, t4343: F, t10811: F, t6037: F, t18444: F, t4364: F, t4366: F, t10846: F, t10885: F, t10888: F, t10891: F, t10900: F, t18491: F, t18495: F, t2730: F, t4362: F, t851: F, t10871: F, t836: F, t18426: F, t221: F, t2485: F, t5978: F, t2484: F, t10552: F, t10554: F, t14317: F, t18261: F, t18262: F, t18265: F, t18267: F, t18300: F, t18301: F, t18308: F, t9278: F, t9308: F, t9316: F, t9329: F, t9333: F) -> (F, F, F, F, F, F, F, F) {
        let (t18500, t18507, t18511, t18515, t18518, t18521) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1474::<F>(t18498, t2477, t828, t5984, t775, t800, t5988, t1548, t4343, t10811, t6037, t18444, t4364, t4366);
        let t18524 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1475::<F>(t10846, t10885, t10888, t10891, t10900, t18491, t18495, t18500, t18507, t18511, t18515, t18518, t18521, t2730, t4362, t851);
        let (t18525, t18527, t18531, t18532, t18534) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1476::<F>(t10871, t836, t18426, t4364, t221, t2485, t5978, t2484, t10552, t10554, t14317, t18261, t18262, t18265, t18267, t18300, t18301, t18308, t9278, t9308, t9316, t9329, t9333);
    (t18500, t18521, t18524, t18525, t18527, t18531, t18532, t18534)
}
