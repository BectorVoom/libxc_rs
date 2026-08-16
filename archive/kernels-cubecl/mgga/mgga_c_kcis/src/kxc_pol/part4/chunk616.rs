//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 616/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk616<F: Float>(t1045: F, t922: F, t3274: F, t1071: F, t347: F, t1103: F, t2630: F, t1104: F, t2635: F, t932: F, t2944: F, t345: F) -> (F, F, F, F, F, F, F) {
    let t3275 = t922 * t1045;
    let t3276 = t3274 * t3275;
    let t3279 = t347 * t1071;
    let t3281 = t1103 * t3279 * t2630;
    let t3285 = t1103 * t1104 * t2635;
    let t3288 = t932 * t347;
    let t3289 = t3288 * t2944;
    let t3290 = t345 * t3289;
    (t3275, t3276, t3281, t3285, t3288, t3289, t3290)
}
