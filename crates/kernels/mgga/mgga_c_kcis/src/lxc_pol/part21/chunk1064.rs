//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1064/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1064<F: Float>(t26623: F, t7589: F, t7580: F, t2140: F, t334: F, t9232: F, t26457: F, t26595: F, t26598: F, t26600: F, t26603: F, t26605: F, t26608: F, t26612: F, t26616: F, t26618: F) -> F {
    let t26624 = t7589 * t26623;
    let t26626 = t7580 * t26623;
    let t26629 = t9232 * t334 * t2140;
    let t26631 = F::cast_from(0.30952962962962962962e-1_f64) * t26457 - F::cast_from(0.185671721767578125e-4_f64) * t26595 - F::cast_from(0.32435763888888888888e-2_f64) * t26598 - F::cast_from(0.32435763888888888888e-2_f64) * t26600 + F::cast_from(0.13901041666666666667e-2_f64) * t26603 + F::cast_from(0.13901041666666666667e-2_f64) * t26605 + F::cast_from(0.18550940104166666667e-3_f64) * t26608 + F::cast_from(0.92754700520833333333e-4_f64) * t26612 + F::cast_from(0.69505208333333333333e-3_f64) * t26616 + F::cast_from(0.69505208333333333333e-3_f64) * t26618 - F::cast_from(0.13901041666666666667e-2_f64) * t26624 - F::cast_from(0.18550940104166666667e-3_f64) * t26626 - F::cast_from(0.69505208333333333333e-3_f64) * t26629;
    t26631
}
