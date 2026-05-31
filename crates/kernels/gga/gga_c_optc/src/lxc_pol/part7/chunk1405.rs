//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1405/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1405<F: Float>(t1200: F, t1205: F, t27875: F, t27888: F, t27901: F, t27914: F, t27918: F, t27921: F, t27926: F, t27935: F, t27936: F, t27942: F, t27959: F, t27972: F, t27985: F, t27998: F, t2881: F, t2886: F, t2887: F, t2900: F, t485: F, t9294: F, t9297: F, t9304: F, t9305: F, t9308: F, t9335: F) -> F {
    let t28002 = (t27875 + t27888 + t27901 + t27914) * t485 - F::cast_from(4.0_f64) * t27918 * t1205 + F::cast_from(12.0_f64) * t27921 * t2887 - F::cast_from(6.0_f64) * t9294 * t2900 - F::cast_from(24.0_f64) * t27926 * t9305 + F::cast_from(24.0_f64) * t9297 * t9308 - F::cast_from(4.0_f64) * t2881 * t9335 + F::cast_from(24.0_f64) * t27935 * t27936 - F::cast_from(36.0_f64) * t9304 * t2887 * t2900 + F::cast_from(6.0_f64) * t2886 * t27942 + F::cast_from(8.0_f64) * t2886 * t1205 * t9335 - t1200 * (t27959 + t27972 + t27985 + t27998);
    t28002
}
