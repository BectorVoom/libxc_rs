//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 727/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk727(t25: f64, t28: f64, t1388: f64, t570: f64, t515: f64, t1298: f64, t2249: f64, t3665: f64, t518: f64, t1302: f64, t3231: f64, t3673: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t29 = t28 <= zeta_threshold;
    let t3698 = t1388 * t1388;
    let t3700 = t570 * t570;
    let t3701 = 1.0_f64 / t3700;
    let t3704 = 1.0_f64 / t515;
    let t3710 = piecewise3(t26, 0.0_f64, -2.0_f64 / 9.0_f64 * t3704 * t3665 + 2.0_f64 / 3.0_f64 * t1298 * t2249);
    let t3711 = 1.0_f64 / t518;
    let t3717 = piecewise3(t29, 0.0_f64, -2.0_f64 / 9.0_f64 * t3711 * t3673 + 2.0_f64 / 3.0_f64 * t1302 * t3231);
    let t3719 = t3710 / 2.0_f64 + t3717 / 2.0_f64;
    (t3698, t3700, t3701, t3704, t3711, t3719)
}
