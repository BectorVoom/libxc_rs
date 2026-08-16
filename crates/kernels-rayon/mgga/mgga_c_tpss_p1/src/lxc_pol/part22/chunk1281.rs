//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1281/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1281(t20329: f64, t20363: f64, t20395: f64, t20646: f64, t3: f64, t1799: f64, t645: f64, t1338: f64, t19040: f64, t3537: f64, t5953: f64, t116: f64, t6323: f64, param_d: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t20648 = t20329 + t20363 + t20395 + t20646;
    let t20649 = t3 * t20648;
    let t20660 = param_d * t20648;
    let t20678 = t645 * t1799;
    let t20679 = t20678 * t1338;
    let t20682 = t19040 * t1338;
    let t20685 = t5953 * t3537;
    let t20690 = t116 * t6323;
    (t20648, t20649, t20660, t20678, t20679, t20682, t20685, t20690)
}
