//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1041/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1041<F: Float>(t157: F, t309: F, t463: F, t694: F, t9114: F, t2407: F, t469: F, t301: F, t11179: F, t1679: F, t467: F, t11883: F, t642: F) -> (F, F, F, F, F, F) {
    let t36495 = t157 * t463 * t309;
    let t36684 = F::cast_from(6.0_f64) * t694 * t9114;
    let t36686 = t2407 * t469;
    let t36689 = F::cast_from(6.0_f64) * t694 * t36686 * t301;
    let t36715 = F::cast_from(2.0_f64) * t1679 * t11179 * t467;
    let t36729 = t642 * t11883;
    (t36495, t36684, t36686, t36689, t36715, t36729)
}
