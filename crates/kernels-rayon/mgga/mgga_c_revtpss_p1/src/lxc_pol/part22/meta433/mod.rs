//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta433 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2060;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta433(t14723: f64, t2662: f64, t2661: f64, t4416: f64, t837: f64, t221: f64, t2485: f64, t4424: f64, t2484: f64, t2652: f64, t4435: f64, t14663: f64, t827: f64, t828: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14724, t14726, t14728, t14730, t14732, t14734, t14736, t14738) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2060(t14723, t2662, t2661, t4416, t837, t221, t2485, t4424, t2484, t2652, t4435, t14663, t827, t828);
    (t14724, t14726, t14728, t14730, t14732, t14734, t14736, t14738)
}
