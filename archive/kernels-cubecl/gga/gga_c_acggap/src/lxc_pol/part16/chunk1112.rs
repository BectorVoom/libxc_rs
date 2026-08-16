//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1112/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1112<F: Float>(t1773: F, t2030: F, t2031: F, t1181: F, t5537: F, t7351: F, t7564: F, t5796: F, t7822: F, t5801: F, t6226: F, t1165: F, t6198: F, t8600: F) -> (F, F, F, F, F, F) {
    let t39402 = t2030 * t1773 * t2031;
    let t39406 = t7564 * t1181 * t7351 * t5537;
    let t39412 = t7822 * t5796;
    let t39414 = t7822 * t5801;
    let t39418 = t7564 * t1181 * t7351 * t6226;
    let t39422 = t7564 * t1165 * t8600 * t6198;
    (t39402, t39406, t39412, t39414, t39418, t39422)
}
