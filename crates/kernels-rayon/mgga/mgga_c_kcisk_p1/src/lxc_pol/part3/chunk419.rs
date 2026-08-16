//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 419/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk419(t116: f64, t3174: f64, t982: f64, t979: f64, t136: f64, t2925: f64, t2927: f64, t2932: f64, t2936: f64, t3064: f64, t3071: f64, t3075: f64, t3078: f64, t3130: f64, t3134: f64, t3142: f64, t856: f64, t934: f64) -> (f64, f64, f64, f64) {
    let t3175 = t116 * t3174;
    let t3176 = t982 * t3175;
    let t3177 = t979 * t3176;
    let t3179 = t2925 * t136 - 0.386e0_f64 * t2927 * t934 + 0.74498e-1_f64 * t2932 * t2936 - 0.193e0_f64 * t856 * t3064 + 0.193e0_f64 * t856 * t2936 + 0.30952962962962962962e-1_f64 * t3071 - 0.2653111111111111111e-1_f64 * t3075 + 0.2653111111111111111e-1_f64 * t3078 + 0.99491666666666666664e-2_f64 * t3130 - 0.19898333333333333333e-1_f64 * t3134 + 0.19898333333333333333e-1_f64 * t3142 - 0.99491666666666666664e-2_f64 * t3177;
    (t3175, t3176, t3177, t3179)
}
