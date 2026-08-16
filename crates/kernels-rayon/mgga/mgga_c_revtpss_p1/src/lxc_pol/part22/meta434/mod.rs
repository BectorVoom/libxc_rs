//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta434 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2061;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2062;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta434(t4343: f64, t854: f64, t236: f64, t807: f64, t124: f64, t14468: f64, t800: f64, t775: f64, t2477: f64, t828: f64, t14712: f64, t14715: f64, t14716: f64, t14722: f64, t14726: f64, t14730: f64, t14734: f64, t14736: f64, t14738: f64, t799: f64, t825: f64, t851: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t14741, t14742, t14744, t14746, t14749) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2061(t4343, t854, t236, t807, t124, t14468, t800, t775);
        let (t14751, t14754) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2062(t14749, t2477, t828, t14712, t14715, t14716, t14722, t14726, t14730, t14734, t14736, t14738, t14744, t14746, t799, t825, t851);
    (t14741, t14742, t14744, t14746, t14749, t14751, t14754)
}
