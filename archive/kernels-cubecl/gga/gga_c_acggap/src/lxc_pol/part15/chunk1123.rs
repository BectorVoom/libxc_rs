//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1123/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1123<F: Float>(t5801: F, t7822: F, t1181: F, t6226: F, t7351: F, t7564: F, t1165: F, t6198: F, t8600: F, t1784: F, t2020: F, t1095: F, t1980: F, t5659: F, t7476: F) -> (F, F, F, F, F) {
    let t39414 = t7822 * t5801;
    let t39418 = t7564 * t1181 * t7351 * t6226;
    let t39422 = t7564 * t1165 * t8600 * t6198;
    let t39427 = t2020 * t1784;
    let t39438 = t1980 * t7476 * t1095 * t5659;
    (t39414, t39418, t39422, t39427, t39438)
}
