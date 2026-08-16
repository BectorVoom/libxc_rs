//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1045/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1045(t1874: f64, t2048: f64, t2041: f64, t35: f64, t88: f64, t22338: f64, t85: f64, t1872: f64, t22610: f64, t22700: f64, t22703: f64, t22705: f64, t22708: f64, t22711: f64, t22713: f64, t22716: f64, t22719: f64) -> (f64, f64, f64, f64, f64) {
    let t22720 = t2048 * t1874;
    let t22721 = 384.0_f64 * t22720;
    let t22723 = t35 * t2041 * t88;
    let t22724 = 1440.0_f64 * t22723;
    let t22726 = 0.19751789702565206229e-1_f64 * t22338 * t85;
    let t22727 = t2048 * t1872;
    let t22728 = 192.0_f64 * t22727;
    let t22729 = t22700 + t22703 + t22705 + t22708 - t22711 - t22713 - t22716 - t22719 - t22721 + t22724 + t22726 + t22610 - t22728;
    (t22721, t22724, t22726, t22728, t22729)
}
