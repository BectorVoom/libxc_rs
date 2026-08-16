//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1300/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1300(t111808: f64, t1268: f64, t12725: f64, t1393: f64, t19289: f64, t19451: f64, t2199: f64, t2202: f64, t2314: f64, t26114: f64, t26179: f64, t28002: f64, t30272: f64, t30274: f64, t30321: f64, t30326: f64, t30534: f64, t30535: f64, t30558: f64, t4028: f64, t4034: f64, t510: f64, t5113: f64, t55943: f64, t652: f64, t7458: f64, t8196: f64, t8260: f64, t8274: f64, t8280: f64, t96683: f64) -> f64 {
    let t112006 = -2.0_f64 * t111808 * t510 * t652 + 2.0_f64 * t1268 * t1393 * t30534 - 2.0_f64 * t19289 * t2199 * t652 - 4.0_f64 * t12725 * t8274 + 2.0_f64 * t19451 * t8196 + 2.0_f64 * t2202 * t55943 + 4.0_f64 * t2202 * t96683 + 2.0_f64 * t2314 * t30535 - 2.0_f64 * t2314 * t30558 - 4.0_f64 * t26114 * t8274 + 4.0_f64 * t26114 * t8280 - 4.0_f64 * t26179 * t8260 - 4.0_f64 * t26179 * t8274 + 4.0_f64 * t28002 * t8196 - 4.0_f64 * t30272 * t7458 - 4.0_f64 * t30274 * t4028 + 4.0_f64 * t30321 * t4028 - 4.0_f64 * t30326 * t7458 + 2.0_f64 * t30535 * t5113 - 2.0_f64 * t30558 * t4034;
    t112006
}
