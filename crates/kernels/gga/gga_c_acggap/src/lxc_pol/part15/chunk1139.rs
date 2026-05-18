//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1139/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1139<F: Float>(t1181: F, t39491: F, t604: F, t7493: F, t1165: F, t5693: F, t7351: F, t8463: F, t1849: F, t360: F, t7575: F, t6209: F) -> (F, F, F, F) {
    let t39705 = t7493 * t1181 * t604 * t39491;
    let t39709 = t8463 * t1165 * t7351 * t5693;
    let t39720 = t7575 * t1181 * t7351 * t1849 * t360;
    let t39724 = t7575 * t1181 * t604 * t6209;
    (t39705, t39709, t39720, t39724)
}
