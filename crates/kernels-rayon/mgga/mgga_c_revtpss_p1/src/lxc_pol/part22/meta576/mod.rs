//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta576 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2432;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta576(t18426: f64, t18525: f64, t4364: f64, t221: f64, t2485: f64, t5978: f64, t2484: f64, t10552: f64, t10554: f64, t14317: f64, t18261: f64, t18262: f64, t18265: f64, t18267: f64, t18300: f64, t18301: f64, t18308: f64, t9278: f64, t9308: f64, t9316: f64, t9329: f64, t9333: f64) -> (f64, f64, f64, f64) {
        let (t18527, t18531, t18532, t18534) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2432(t18426, t18525, t4364, t221, t2485, t5978, t2484, t10552, t10554, t14317, t18261, t18262, t18265, t18267, t18300, t18301, t18308, t9278, t9308, t9316, t9329, t9333);
    (t18527, t18531, t18532, t18534)
}
