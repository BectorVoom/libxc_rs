//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 645/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk645(t4054: f64, t1552: f64, t435: f64, t128: f64, t505: f64, t188: f64, t516: f64, t424: f64, t515: f64, t3668: f64, t653: f64, t1870: f64, t442: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t4055 = t4054 * pi;
    let t4059 = t435 * t1552;
    let t4296 = t128 * t505;
    let t4533 = t516 * t188;
    let t4538 = t424 * t515;
    let t4605 = t3668 * t653;
    let t4644 = t1870 * t442;
    (t4055, t4059, t4296, t4533, t4538, t4605, t4644)
}
