//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1405/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1405(t1200: f64, t1205: f64, t27875: f64, t27888: f64, t27901: f64, t27914: f64, t27918: f64, t27921: f64, t27926: f64, t27935: f64, t27936: f64, t27942: f64, t27959: f64, t27972: f64, t27985: f64, t27998: f64, t2881: f64, t2886: f64, t2887: f64, t2900: f64, t485: f64, t9294: f64, t9297: f64, t9304: f64, t9305: f64, t9308: f64, t9335: f64) -> f64 {
    let t28002 = (t27875 + t27888 + t27901 + t27914) * t485 - 4.0_f64 * t27918 * t1205 + 12.0_f64 * t27921 * t2887 - 6.0_f64 * t9294 * t2900 - 24.0_f64 * t27926 * t9305 + 24.0_f64 * t9297 * t9308 - 4.0_f64 * t2881 * t9335 + 24.0_f64 * t27935 * t27936 - 36.0_f64 * t9304 * t2887 * t2900 + 6.0_f64 * t2886 * t27942 + 8.0_f64 * t2886 * t1205 * t9335 - t1200 * (t27959 + t27972 + t27985 + t27998);
    t28002
}
