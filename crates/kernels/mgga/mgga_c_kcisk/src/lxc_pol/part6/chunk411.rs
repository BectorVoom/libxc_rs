//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 411/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk411<F: Float>(t156: F, t969: F, t23: F, t916: F, t6: F, t161: F, t3042: F, t7: F, t171: F, t213: F, t967: F, t151: F, t175: F, t2925: F, t3082: F, t3087: F, t3088: F, t3107: F, t60: F, t852: F, t945: F, t955: F, t972: F) -> (F, F, F, F, F, F, F, F) {
    let t3114 = t156 * t969;
    let t3117 = 1.0 / t23 / t916;
    let t3118 = t6 * t3117;
    let t3119 = t161 * t3118;
    let t3122 = t7 * t3042;
    let t3123 = t171 * t3122;
    let t3125 = -0.11955719325063177623e-1 * t967 + 0.40985e-2 * t3114 - 0.10566666666666666667e-2 * t3119 + 0.3884654180847230157e-4 * t213 - 0.420109375e-5 * t3123;
    let t3127 = 0.23426533963880895498e-2 * t967 * t151 + 0.46853067927761790996e-2 * t3082 * t955 + 0.70279601891642686494e-2 * t3087 * t3088 - 0.23426533963880895498e-2 * t945 * t3107 - t2925 * t175 - 2.0 * t852 * t972 - t60 * t3125;
    (t3114, t3117, t3118, t3119, t3122, t3123, t3125, t3127)
}
