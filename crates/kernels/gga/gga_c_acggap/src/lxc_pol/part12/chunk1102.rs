//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1102/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1102<F: Float>(t1165: F, t20433: F, t2068: F, t7351: F, t31362: F, t8956: F, t525: F, t839: F, t604: F, t7337: F, t7839: F, t8962: F) -> (F, F, F, F, F) {
    let t35614 = t2068 * t1165 * t7351 * t20433;
    let t35616 = t31362 * t8956;
    let t35618 = t525 * t839;
    let t35621 = t7337 * t1165 * t604 * t35618;
    let t35623 = t7839 * t8962;
    (t35614, t35616, t35618, t35621, t35623)
}
