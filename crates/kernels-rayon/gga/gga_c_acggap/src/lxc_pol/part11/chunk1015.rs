//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1015/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1015(t33994: f64, t7839: f64, t8779: f64, t4991: f64, t7822: f64, t5192: f64, t2068: f64, t4680: f64, t8778: f64, t2001: f64, t5014: f64, t1089: f64, t535: f64, t7553: f64, t7554: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t33995 = 0.7145669686344956162e-3_f64 * t33994;
    let t33996 = t7839 * t8779;
    let t33997 = 0.42874018118069736972e-3_f64 * t33996;
    let t33998 = t7822 * t4991;
    let t34000 = t7822 * t5192;
    let t34003 = t2068 * t4680 * t8778;
    let t34005 = t2001 * t5014;
    let t34009 = t7553 * t1089 * t535 * t7554;
    (t33995, t33997, t33998, t34000, t34003, t34005, t34009)
}
