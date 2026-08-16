//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta256 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1090;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta256(t11349: f64, t11378: f64, t935: f64, t915: f64, t2922: f64, t913: f64, t275: f64, t290: f64, t2925: f64, t11300: f64, t3022: f64, t3030: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11379, t11380, t11382, t11384, t11385, t11387, t11388, t11390, t11392) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1090(t11349, t11378, t935, t915, t2922, t913, t275, t290, t2925, t11300, t3022, t3030);
    (t11379, t11380, t11382, t11384, t11385, t11387, t11388, t11390, t11392)
}
