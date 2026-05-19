//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 588/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk588<F: Float>(t3073: F, t3074: F, t2943: F, t308: F, t1042: F, t932: F, t2917: F, t2919: F, t2922: F, t2925: F, t2928: F, t2945: F, t2953: F) -> (F, F, F, F, F) {
    let t3075 = t3073 * t3074;
    let t3078 = t2943 * t308;
    let t3081 = t932 * t1042;
    let t3088 = F::cast_from(0.55033333333333333333e-2_f64) * t2917;
    let t3093 = -F::new(0.991e-2) * t2945 + F::new(0.1982e-1) * t2953 + t3088 + F::cast_from(0.27516666666666666666e-2_f64) * t2919 - F::cast_from(0.27516666666666666667e-2_f64) * t2922 + F::new(0.8255e-2) * t2925 - F::new(0.41275e-2) * t2928;
    (t3075, t3078, t3081, t3088, t3093)
}
