//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 773/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk773(t2603: f64, t2609: f64, t2623: f64, t2640: f64, t2655: f64, t3835: f64, t7350: f64, t7355: f64, t7360: f64, t7366: f64, t7372: f64, t7376: f64, t7379: f64, t7383: f64, t7386: f64, t7389: f64, t7395: f64, t7399: f64, t7403: f64, t7407: f64, t7410: f64, t7413: f64, t862: f64, t867: f64) -> f64 {
    let t7415 = -0.1420012659563261767e0_f64 * t2640 * t7350 - 0.10866451862235947318e-1_f64 * t3835 * t7355 + 0.90553765518632894319e-2_f64 * t3835 * t7360 - 0.56800506382530470682e0_f64 * t2655 * t2609 + 0.71000632978163088351e-1_f64 * t7366 + 0.17715845405452227366e4_f64 * t7372 * t7376 + 0.10629507243271336419e5_f64 * t7379 * t7383 - 0.10629507243271336419e5_f64 * t7386 * t7389 + t2623 * t2603 / 18.0_f64 - t7395 / 144.0_f64 + t862 * t7399 / 48.0_f64 - t7403 / 432.0_f64 - t862 * t7407 / 36.0_f64 + 11.0_f64 / 108.0_f64 * t7410 * t867 - t7413 / 54.0_f64;
    t7415
}
