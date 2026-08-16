//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1710/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1710(t221: f64, t3979: f64, t6816: f64, t3978: f64, t3989: f64, t6880: f64, t22025: f64, t543: f64, t3992: f64, t2661: f64, t1370: f64, t13779: f64, t13781: f64, t13797: f64, t1410: f64, t22038: f64, t22041: f64, t22044: f64, t22048: f64, t22052: f64, t5671: f64, t9735: f64) -> (f64, f64, f64) {
    let t22056 = t3979 * t221 * t6816;
    let t22057 = t3978 * t22056;
    let t22059 = t3989 * t6880;
    let t22061 = t22025 * t543;
    let t22062 = t3992 * t22061;
    let t22063 = t2661 * t22062;
    let t22065 = -0.15244095330869239812e-3_f64 * t13779 - 0.45351183609335988442e-1_f64 * t13781 + 7.0_f64 / 144.0_f64 * t22038 - t1370 * t22041 / 48.0_f64 - 7.0_f64 / 48.0_f64 * t22044 - t9735 - 0.17149607247227894789e-2_f64 * t5671 * t22048 + t13797 - 0.85748036236139473944e-3_f64 * t1410 * t22052 - 0.50820002809285328225e-4_f64 * t22057 - 0.20007875121765877254e-1_f64 * t22059 + 0.71456696863449561619e-5_f64 * t22063;
    (t22056, t22061, t22065)
}
