//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 811/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk811<F: Float>(t265: F, t7620: F, t241: F, t2449: F, t778: F, t800: F, t2378: F, t2410: F, t2415: F, t774: F, t2419: F, t7339: F, t7346: F, t7348: F, t7499: F, t7507: F, t7509: F, t7608: F) -> (F, F, F, F, F, F, F, F) {
    let t7621 = t7620 * t265;
    let t7623 = F::cast_from(0.19751789702565206229e-1_f64) * t241 * t7621;
    let t7624 = t2449 * t778;
    let t7626 = F::cast_from(3.0_f64) * t7624 * t800;
    let t7628 = F::cast_from(3.0_f64) * t2378 * t2410;
    let t7629 = t774 * t2415;
    let t7631 = F::cast_from(0.48245472966453314466e2_f64) * t7629 * t2419;
    let t7632 = -t7499 - t7507 - t7339 + t7346 - t7509 - t7608 + t7348 + t7623 + t7626 + t7628 + t7631;
    (t7621, t7623, t7624, t7626, t7628, t7629, t7631, t7632)
}
