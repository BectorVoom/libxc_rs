//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1100/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1100<F: Float>(t1460: F, t30159: F, t355: F, t3706: F, t7842: F, t30374: F, t8606: F, t1181: F, t4342: F, t7351: F, t7575: F, t7426: F, t7569: F, t8480: F) -> (F, F, F, F) {
    let t35585 = t30159 * t7842 * t3706 * t355 * t1460;
    let t35587 = t30374 * t8606;
    let t35591 = t7575 * t1181 * t7351 * t4342;
    let t35594 = t7426 * t8480 * t7569;
    (t35585, t35587, t35591, t35594)
}
