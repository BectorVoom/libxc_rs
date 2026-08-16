//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 978/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk978(t22479: f64, t89: f64, t2040: f64, t31540: f64, t7050: f64, t2314: f64, t31747: f64, t531: f64, t8639: f64, t1983: f64, t22596: f64, t22581: f64, t8607: f64) -> (f64, f64, f64, f64, f64) {
    let t115252 = t89 * t22479;
    let t115254 = 2.0_f64 * t115252 * t2040;
    let t115256 = 4.0_f64 * t31540 * t7050;
    let t115261 = 4.0_f64 * t2314 * t31747;
    let t115262 = t531 * t8639;
    let t115265 = 6.0_f64 * t1983 * t115262 * t22596;
    let t115271 = 2.0_f64 * t8607 * t22581;
    (t115254, t115256, t115261, t115265, t115271)
}
