//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1297/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1297(t111415: f64, t1268: f64, t12725: f64, t1849: f64, t19289: f64, t20098: f64, t2180: f64, t2183: f64, t2314: f64, t26114: f64, t26117: f64, t26179: f64, t28002: f64, t28007: f64, t30180: f64, t30181: f64, t30195: f64, t30201: f64, t30433: f64, t30454: f64, t4034: f64, t510: f64, t55943: f64, t6287: f64, t652: f64, t7458: f64, t7676: f64, t8143: f64, t8148: f64, t8150: f64, t8231: f64, t8235: f64, t8237: f64, t96356: f64) -> f64 {
    let t111592 = -2.0_f64 * t2314 * t30433 - 2.0_f64 * t4034 * t30433 - 2.0_f64 * t652 * t19289 * t2180 - 2.0_f64 * t652 * t510 * t111415 + 2.0_f64 * t55943 * t2183 - 2.0_f64 * t652 * t6287 * t8143 + 4.0_f64 * t12725 * t8235 + 2.0_f64 * t1268 * t2180 * t20098 + 4.0_f64 * t96356 * t2183 + 4.0_f64 * t28002 * t8148 + 4.0_f64 * t7676 * t30181 + 4.0_f64 * t7676 * t30201 + 4.0_f64 * t12725 * t8237 - 4.0_f64 * t26179 * t8231 - 4.0_f64 * t7458 * t30195 + 4.0_f64 * t26114 * t8237 + 4.0_f64 * t26117 * t8237 + 2.0_f64 * t28007 * t8150 + 2.0_f64 * t2314 * t30454 + 4.0_f64 * t1268 * t30180 * t1849;
    t111592
}
