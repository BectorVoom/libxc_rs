//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 512/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk512(t2073: f64, t2074: f64, t588: f64, t99: f64, t633: f64, t100: f64, t1990: f64, t107: f64, t636: f64, t108: f64, t101: f64, t105: f64, t631: f64, t634: f64, t97: f64, tau0: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2075 = t2073 * t2074;
    let t2078 = tau0 * t588;
    let t2083 = 1.0_f64 / t99;
    let t2084 = t633 * t633;
    let t2085 = t2083 * t2084;
    let t2088 = t100 * t1990;
    let t2091 = 1.0_f64 / t107;
    let t2092 = t636 * t636;
    let t2093 = t2091 * t2092;
    let t2096 = -t1990;
    let t2097 = t108 * t2096;
    let t2100 = 40.0_f64 / 9.0_f64 * t2078 * t101 - 50.0_f64 / 9.0_f64 * t631 * t634 + 10.0_f64 / 9.0_f64 * t97 * t2085 + 5.0_f64 / 3.0_f64 * t97 * t2088 + 10.0_f64 / 9.0_f64 * t105 * t2093 + 5.0_f64 / 3.0_f64 * t105 * t2097;
    (t2075, t2078, t2083, t2084, t2091, t2092, t2093, t2096, t2097, t2100)
}
