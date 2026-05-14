//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 740/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk740<F: Float>(t7402: F, t862: F, t2548: F, t7298: F, t6534: F, t322: F, t2573: F, t861: F, t2623: F, t2626: F, t2603: F, t2609: F, t2640: F, t2655: F, t3835: F, t7350: F, t7355: F, t7360: F, t7366: F, t7372: F, t7376: F, t7379: F, t7383: F, t7386: F, t7389: F, t7395: F, t7399: F, t867: F) -> (F, F, F, F, F) {
    let t7403 = t862 * t7402;
    let t7405 = t2548 * t7298;
    let t7406 = t7405 * t6534;
    let t7407 = t322 * t7406;
    let t7410 = t2573 * t861;
    let t7413 = t2623 * t2626;
    let t7415 = -0.1420012659563261767e0 * t2640 * t7350 - 0.10866451862235947318e-1 * t3835 * t7355 + 0.90553765518632894319e-2 * t3835 * t7360 - 0.56800506382530470682e0 * t2655 * t2609 + 0.71000632978163088351e-1 * t7366 + 0.17715845405452227366e4 * t7372 * t7376 + 0.10629507243271336419e5 * t7379 * t7383 - 0.10629507243271336419e5 * t7386 * t7389 + t2623 * t2603 / 18.0 - t7395 / 144.0 + t862 * t7399 / 48.0 - t7403 / 432.0 - t862 * t7407 / 36.0 + 11.0 / 108.0 * t7410 * t867 - t7413 / 54.0;
    (t7405, t7406, t7407, t7410, t7415)
}
