//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 425/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk425<F: Float>(t156: F, t969: F, t23: F, t916: F, t6: F, t161: F, t3042: F, t7: F, t171: F, t213: F, t967: F, t151: F, t175: F, t2925: F, t3082: F, t3087: F, t3088: F, t3107: F, t60: F, t852: F, t945: F, t955: F, t972: F) -> (F, F, F, F, F, F, F, F) {
    let t3114 = t156 * t969;
    let t3117 = F::cast_from(1.0_f64) / t23 / t916;
    let t3118 = t6 * t3117;
    let t3119 = t161 * t3118;
    let t3122 = t7 * t3042;
    let t3123 = t171 * t3122;
    let t3125 = -F::cast_from(0.11955719325063177623e-1_f64) * t967 + F::cast_from(0.40985e-2_f64) * t3114 - F::cast_from(0.10566666666666666667e-2_f64) * t3119 + F::cast_from(0.3884654180847230157e-4_f64) * t213 - F::cast_from(0.420109375e-5_f64) * t3123;
    let t3127 = F::cast_from(0.23426533963880895498e-2_f64) * t967 * t151 + F::cast_from(0.46853067927761790996e-2_f64) * t3082 * t955 + F::cast_from(0.70279601891642686494e-2_f64) * t3087 * t3088 - F::cast_from(0.23426533963880895498e-2_f64) * t945 * t3107 - t2925 * t175 - F::cast_from(2.0_f64) * t852 * t972 - t60 * t3125;
    (t3114, t3117, t3118, t3119, t3122, t3123, t3125, t3127)
}
