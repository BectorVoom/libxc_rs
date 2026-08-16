//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1027/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1027(t11018: f64, t2515: f64, t141: f64, t11022: f64, t11008: f64, t8633: f64, t11031: f64, t861: f64, t11035: f64, t11004: f64, t11040: f64, t854: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11055 = t2515 * t11018;
    let t11056 = t141 * t11055;
    let t11058 = t2515 * t11022;
    let t11059 = t141 * t11058;
    let t11061 = t8633 * t11008;
    let t11062 = t141 * t11061;
    let t11064 = t861 * t11031;
    let t11065 = t141 * t11064;
    let t11067 = t861 * t11035;
    let t11068 = t141 * t11067;
    let t11071 = 0.39862222222222222222e0_f64 * t11004;
    let t11080 = t854 * t11040;
    (t11056, t11059, t11062, t11065, t11068, t11071, t11080)
}
