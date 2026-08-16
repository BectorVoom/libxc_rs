//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2072/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2072(t28177: f64, t7235: f64, t28056: f64, t4254: f64, t5517: f64, t651: f64, t7002: f64, t2028: f64, t27980: f64, t13790: f64, t4102: f64, t685: f64, t72: f64) -> (f64, f64, f64, f64, f64) {
    let t97661 = 6.0_f64 * t7235 * t28177;
    let t97663 = 4.0_f64 * t4254 * t28056;
    let t97666 = 4.0_f64 * t651 * t5517 * t7002;
    let t97676 = t2028 * t27980;
    let t97680 = t13790 * t72 * t685 * t4102;
    (t97661, t97663, t97666, t97676, t97680)
}
