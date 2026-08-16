//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1300/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1300<F: Float>(t111808: F, t1268: F, t12725: F, t1393: F, t19289: F, t19451: F, t2199: F, t2202: F, t2314: F, t26114: F, t26179: F, t28002: F, t30272: F, t30274: F, t30321: F, t30326: F, t30534: F, t30535: F, t30558: F, t4028: F, t4034: F, t510: F, t5113: F, t55943: F, t652: F, t7458: F, t8196: F, t8260: F, t8274: F, t8280: F, t96683: F) -> F {
    let t112006 = -F::cast_from(2.0_f64) * t111808 * t510 * t652 + F::cast_from(2.0_f64) * t1268 * t1393 * t30534 - F::cast_from(2.0_f64) * t19289 * t2199 * t652 - F::cast_from(4.0_f64) * t12725 * t8274 + F::cast_from(2.0_f64) * t19451 * t8196 + F::cast_from(2.0_f64) * t2202 * t55943 + F::cast_from(4.0_f64) * t2202 * t96683 + F::cast_from(2.0_f64) * t2314 * t30535 - F::cast_from(2.0_f64) * t2314 * t30558 - F::cast_from(4.0_f64) * t26114 * t8274 + F::cast_from(4.0_f64) * t26114 * t8280 - F::cast_from(4.0_f64) * t26179 * t8260 - F::cast_from(4.0_f64) * t26179 * t8274 + F::cast_from(4.0_f64) * t28002 * t8196 - F::cast_from(4.0_f64) * t30272 * t7458 - F::cast_from(4.0_f64) * t30274 * t4028 + F::cast_from(4.0_f64) * t30321 * t4028 - F::cast_from(4.0_f64) * t30326 * t7458 + F::cast_from(2.0_f64) * t30535 * t5113 - F::cast_from(2.0_f64) * t30558 * t4034;
    t112006
}
