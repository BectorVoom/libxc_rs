//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1055/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1055<F: Float>(t19114: F, t6279: F, t3598: F, t476: F, t2250: F, t979: F, t4265: F, t6300: F, t442: F, t5864: F, t1056: F, t1471: F, t140: F, t299: F, t6303: F, t1173: F, t1460: F) -> (F, F, F, F, F, F, F) {
    let t21140 = t6279 * t19114;
    let t21145 = t476 * t3598;
    let t21152 = t979 * t2250;
    let t21154 = t4265 * t6300;
    let t21156 = t5864 * t442;
    let t21158 = t1471 * t21156 * t1056;
    let t21163 = 0.53062222222222222222e-1 * t140 * t299 * t6303;
    let t21164 = t1460 * t1173;
    (t21140, t21145, t21152, t21154, t21158, t21163, t21164)
}
