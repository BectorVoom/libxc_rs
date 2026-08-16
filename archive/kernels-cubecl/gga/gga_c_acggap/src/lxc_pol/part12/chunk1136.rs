//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1136/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1136<F: Float>(t30689: F, t5286: F, t1165: F, t2068: F, t20972: F, t7351: F, t1181: F, t22107: F, t604: F, t8463: F, t4257: F, t22275: F, t7493: F) -> (F, F, F, F, F) {
    let t36177 = t30689 * t5286;
    let t36181 = t2068 * t1165 * t7351 * t20972;
    let t36186 = t8463 * t1181 * t604 * t22107;
    let t36190 = t8463 * t1165 * t7351 * t4257;
    let t36194 = t7493 * t1181 * t604 * t22275;
    (t36177, t36181, t36186, t36190, t36194)
}
