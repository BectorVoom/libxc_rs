//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1181/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1181<F: Float>(t38145: F, t6085: F, t9242: F, t6093: F, t9246: F, t2201: F, t3216: F, t3319: F, t3320: F, t10698: F, t12523: F, t3602: F, t39922: F, t8081: F) -> (F, F, F, F, F) {
    let t43441 = t6085 * t38145 * t9242;
    let t43447 = t6093 * t38145 * t9246;
    let t43451 = t2201 * t3319 * t3320 * t3216;
    let t43454 = t10698 * t12523;
    let t43459 = t39922 * t3602 * t8081;
    (t43441, t43447, t43451, t43454, t43459)
}
