//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 773/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk773<F: Float>(t2603: F, t2609: F, t2623: F, t2640: F, t2655: F, t3835: F, t7350: F, t7355: F, t7360: F, t7366: F, t7372: F, t7376: F, t7379: F, t7383: F, t7386: F, t7389: F, t7395: F, t7399: F, t7403: F, t7407: F, t7410: F, t7413: F, t862: F, t867: F) -> F {
    let t7415 = -F::cast_from(0.1420012659563261767e0_f64) * t2640 * t7350 - F::cast_from(0.10866451862235947318e-1_f64) * t3835 * t7355 + F::cast_from(0.90553765518632894319e-2_f64) * t3835 * t7360 - F::cast_from(0.56800506382530470682e0_f64) * t2655 * t2609 + F::cast_from(0.71000632978163088351e-1_f64) * t7366 + F::cast_from(0.17715845405452227366e4_f64) * t7372 * t7376 + F::cast_from(0.10629507243271336419e5_f64) * t7379 * t7383 - F::cast_from(0.10629507243271336419e5_f64) * t7386 * t7389 + t2623 * t2603 / F::cast_from(18.0_f64) - t7395 / F::cast_from(144.0_f64) + t862 * t7399 / F::cast_from(48.0_f64) - t7403 / F::cast_from(432.0_f64) - t862 * t7407 / F::cast_from(36.0_f64) + F::cast_from(11.0_f64) / F::cast_from(108.0_f64) * t7410 * t867 - t7413 / F::cast_from(54.0_f64);
    t7415
}
