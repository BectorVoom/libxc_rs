//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1265/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1265<F: Float>(t26278: F, t26289: F, t26300: F, t26306: F, t26309: F, t26406: F, t26409: F, t26412: F, t26415: F, t26419: F, t26443: F, t26446: F, t1200: F, t1205: F, t27875: F, t27888: F, t27901: F, t27914: F, t27918: F, t27921: F, t27926: F, t27935: F, t27936: F, t27942: F, t27959: F, t27972: F, t27985: F, t2881: F, t2886: F, t2887: F, t2900: F, t485: F, t9294: F, t9297: F, t9304: F, t9305: F, t9308: F, t9335: F) -> (F,) {
    let t27998 = 0.12586666666666666667e4 * t26406 - 0.94399999999999999998e3 * t26409 - 0.78666666666666666666e2 * t26412 + 1888.0 * t26415 + 0.47199999999999999999e3 * t26419 - 0.4846111111111111111e4 * t26278 + 17446.0 * t26289 - 26169.0 * t26300 - 0.58153333333333333333e4 * t26306 + 0.19384444444444444445e4 * t26309 + 0.94399999999999999998e3 * t26443 - 2832.0 * t26446;
    let t28002 = (t27875 + t27888 + t27901 + t27914) * t485 - 4.0 * t27918 * t1205 + 12.0 * t27921 * t2887 - 6.0 * t9294 * t2900 - 24.0 * t27926 * t9305 + 24.0 * t9297 * t9308 - 4.0 * t2881 * t9335 + 24.0 * t27935 * t27936 - 36.0 * t9304 * t2887 * t2900 + 6.0 * t2886 * t27942 + 8.0 * t2886 * t1205 * t9335 - t1200 * (t27959 + t27972 + t27985 + t27998);
    (t28002,)
}
