//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta28 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk216;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk217;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk218;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk219;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk220;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk221;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk222;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta28(t549: f64, t241: f64, t247: f64, t217: f64, t535: f64, t548: f64, t225: f64, t546: f64, t213: f64, t149: f64, t198: f64, t522: f64, t524: f64, t532: f64, t118: f64, t508: f64, t511: f64, param_d: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t550 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk216(t549);
        let t555 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk217(t241, t550, t247, t217, t535, t548);
        let t556 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk218(t225, t555);
        let (t557, t560, t561) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk219(t546, t555, t213);
        let (t562, t565, t566) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk220(t556, t561, t213);
        let t569 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk221(t149, t198, t522, t524, t532, t566);
        let (t571, t572) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk222(t118, t508, t511, t569, param_d);
    (t550, t555, t556, t557, t560, t561, t562, t565, t566, t569, t571, t572)
}
