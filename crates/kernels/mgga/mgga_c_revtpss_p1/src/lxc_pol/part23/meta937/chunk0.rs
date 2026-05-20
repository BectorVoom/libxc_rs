//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3081/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3081<F: Float>(t1145: F, t141: F, t81226: F, t24294: F, t698: F, t24288: F, t24291: F, t68262: F, t68277: F, t68312: F, t68332: F, t68334: F, t68336: F, t68368: F, t68370: F) -> (F, F, F, F, F) {
    let t81423 = t141 * t1145 * t81226;
    let t81425 = t698 * t24294;
    let t81427 = t698 * t24288;
    let t81429 = t698 * t24291;
    let t81437 = -F::cast_from(0.33218518518518518518e0_f64) * t68262 - F::cast_from(0.59793333333333333334e0_f64) * t68277 + F::cast_from(0.82156666666666666667e-1_f64) * t81423 - F::cast_from(0.54771111111111111111e-1_f64) * t81425 + F::cast_from(0.10954222222222222222e0_f64) * t81427 - F::cast_from(0.32862666666666666666e0_f64) * t81429 + F::cast_from(0.5477111111111111111e-1_f64) * t68312 + F::cast_from(0.19931111111111111111e0_f64) * t68332 + F::cast_from(0.39862222222222222222e0_f64) * t68334 + F::cast_from(0.11958666666666666667e1_f64) * t68336 - F::cast_from(0.32862666666666666666e0_f64) * t68368 - F::cast_from(0.73028148148148148146e-1_f64) * t68370;
    (t81423, t81425, t81427, t81429, t81437)
}
