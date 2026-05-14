//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 566/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk566<F: Float>(t3073: F, t3074: F, t2943: F, t308: F, t1042: F, t932: F, t2917: F, t2919: F, t2922: F, t2925: F, t2928: F, t2945: F, t2953: F, t1036: F, t245: F, t2944: F, t2952: F, t934: F) -> (F, F, F, F, F, F) {
    let t3075 = t3073 * t3074;
    let t3078 = t2943 * t308;
    let t3081 = t932 * t1042;
    let t3088 = 0.55033333333333333333e-2 * t2917;
    let t3093 = -0.991e-2 * t2945 + 0.1982e-1 * t2953 + t3088 + 0.27516666666666666666e-2 * t2919 - 0.27516666666666666667e-2 * t2922 + 0.8255e-2 * t2925 - 0.41275e-2 * t2928;
    let t3096 = -t3078 * t2944 / 8.0 + t3081 * t934 / 2.0 + t1036 * t2952 / 4.0 + t245 * t3093 / 2.0;
    (t3075, t3078, t3081, t3088, t3093, t3096)
}
