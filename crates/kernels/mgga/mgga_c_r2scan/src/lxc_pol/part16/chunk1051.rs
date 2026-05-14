//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1051/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1051<F: Float>(t11780: F, t2207: F, t3606: F, t10760: F, t22820: F, t29279: F, t29726: F, t6535: F, t11720: F, t26282: F, t1058: F, t1060: F, t8629: F, t39628: F, t39630: F, t39635: F, t39640: F, t39673: F, t41478: F, t41480: F) -> (F,) {
    let t43195 = t2207 * t11780 * t3606;
    let t43200 = t22820 * t10760 * t29279;
    let t43203 = t6535 * t10760 * t29726;
    let t43205 = t26282 * t11720;
    let t43209 = t2207 * t1058 * t1060 * t8629;
    let t43211 = 0.13099107994629972538e-1 * t43195 + t39628 + t39630 - 0.25426783770825854452e1 * t39635 - t41478 - 0.32927245914677557992e-1 * t39640 + t41480 + 0.13099107994629972538e-1 * t43200 - 0.87327386630866483584e-2 * t43203 - t39673 - 0.13099107994629972538e-1 * t43205 + 0.65495539973149862688e-2 * t43209;
    (t43211,)
}
