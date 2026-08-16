//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2445/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2445(t11804: f64, t11921: f64, t247: f64, t4837: f64, t1063: f64, t11169: f64, t3109: f64, t1011: f64, t11758: f64, t140: f64, t11823: f64, t11828: f64) -> (f64, f64, f64, f64, f64) {
    let t42487 = t4837 * t247 * t11921 * t11804;
    let t42496 = t1063 * t247 * t3109 * t11169;
    let t42499 = t1011 * t140 * t11758;
    let t42506 = t1011 * t140 * t11823;
    let t42516 = t1011 * t140 * t11828;
    (t42487, t42496, t42499, t42506, t42516)
}
