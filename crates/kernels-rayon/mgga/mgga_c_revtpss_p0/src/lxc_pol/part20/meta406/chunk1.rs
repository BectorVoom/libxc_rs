//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1502/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1502(t1011: f64, t1012: f64, t1015: f64, t1063: f64, t1066: f64, t11829: f64, t11853: f64, t11913: f64, t247: f64, t3188: f64, t3241: f64, t39443: f64, t39457: f64, t41271: f64, t41318: f64, t42496: f64, t42499: f64, t42506: f64, t42508: f64, t42516: f64, t42518: f64) -> f64 {
    let t42529 = -0.76220476654346199062e-2_f64 * t1063 * t247 * t11853 * t41271 + 0.38110238327173099531e-3_f64 * t42496 + t42499 / 216.0_f64 + t1011 * t1012 * t1015 * t39457 / 288.0_f64 + 7.0_f64 / 486.0_f64 * t42506 - 7.0_f64 / 54.0_f64 * t1011 * t1012 * t42508 * t39443 + 8.0_f64 / 27.0_f64 * t3241 * t11829 - t42516 / 27.0_f64 + t1011 * t1012 * t42518 * t39443 / 6.0_f64 - 0.85748036236139473944e-3_f64 * t1063 * t247 * t1066 * t41318 - 0.57165357490759649296e-2_f64 * t3188 * t11913;
    t42529
}
