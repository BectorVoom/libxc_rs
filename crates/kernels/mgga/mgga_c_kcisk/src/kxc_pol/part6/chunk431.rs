//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 431/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk431<F: Float>(t116: F, t3174: F, t982: F, t979: F, t136: F, t2925: F, t2927: F, t2932: F, t2936: F, t3064: F, t3071: F, t3075: F, t3078: F, t3130: F, t3134: F, t3142: F, t856: F, t934: F) -> (F, F, F, F) {
    let t3175 = t116 * t3174;
    let t3176 = t982 * t3175;
    let t3177 = t979 * t3176;
    let t3179 = t2925 * t136 - F::new(0.386e0) * t2927 * t934 + F::new(0.74498e-1) * t2932 * t2936 - F::new(0.193e0) * t856 * t3064 + F::new(0.193e0) * t856 * t2936 + F::cast_from(0.30952962962962962962e-1_f64) * t3071 - F::cast_from(0.2653111111111111111e-1_f64) * t3075 + F::cast_from(0.2653111111111111111e-1_f64) * t3078 + F::cast_from(0.99491666666666666664e-2_f64) * t3130 - F::cast_from(0.19898333333333333333e-1_f64) * t3134 + F::cast_from(0.19898333333333333333e-1_f64) * t3142 - F::cast_from(0.99491666666666666664e-2_f64) * t3177;
    (t3175, t3176, t3177, t3179)
}
