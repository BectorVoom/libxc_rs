//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1404/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1404<F: Float>(t26280: F, t26284: F, t26293: F, t26296: F, t26304: F, t26311: F, t26319: F, t26324: F, t26388: F, t26394: F, t26396: F, t26278: F, t26289: F, t26300: F, t26306: F, t26309: F, t26406: F, t26409: F, t26412: F, t26415: F, t26419: F, t26443: F, t26446: F) -> (F, F) {
    let t27985 = -F::cast_from(0.10488888888888888889e3_f64) * t26388 + F::cast_from(0.58153333333333333332e4_f64) * t26319 - F::cast_from(0.19384444444444444444e4_f64) * t26324 - F::cast_from(0.12586666666666666667e4_f64) * t26394 + F::cast_from(0.20977777777777777778e3_f64) * t26396 + F::cast_from(0.58153333333333333332e4_f64) * t26280 - F::cast_from(17446.0_f64) * t26284 - F::cast_from(0.14538333333333333333e4_f64) * t26293 + F::cast_from(17446.0_f64) * t26296 + F::cast_from(0.43614999999999999999e4_f64) * t26304 - F::cast_from(0.38768888888888888889e4_f64) * t26311;
    let t27998 = F::cast_from(0.12586666666666666667e4_f64) * t26406 - F::cast_from(0.94399999999999999998e3_f64) * t26409 - F::cast_from(0.78666666666666666666e2_f64) * t26412 + F::cast_from(1888.0_f64) * t26415 + F::cast_from(0.47199999999999999999e3_f64) * t26419 - F::cast_from(0.4846111111111111111e4_f64) * t26278 + F::cast_from(17446.0_f64) * t26289 - F::cast_from(26169.0_f64) * t26300 - F::cast_from(0.58153333333333333333e4_f64) * t26306 + F::cast_from(0.19384444444444444445e4_f64) * t26309 + F::cast_from(0.94399999999999999998e3_f64) * t26443 - F::cast_from(2832.0_f64) * t26446;
    (t27985, t27998)
}
