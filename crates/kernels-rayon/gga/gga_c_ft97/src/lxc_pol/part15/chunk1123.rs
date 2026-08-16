//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1123/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1123(t18043: f64, t5046: f64, t5038: f64, t41447: f64, t420: f64, t701: f64, t88252: f64, t1107: f64, t207: f64, t14: f64, t228: f64, t231: f64) -> (f64, f64, f64, f64, f64) {
    let t88579 = t18043 * t5046;
    let t88581 = t18043 * t5038;
    let t88585 = t701 * t420 * t41447 * t88252;
    let t88593 = 1.0_f64 / t207 / t1107;
    let t88596 = t228 * t88593 * t14 * t231;
    (t88579, t88581, t88585, t88593, t88596)
}
