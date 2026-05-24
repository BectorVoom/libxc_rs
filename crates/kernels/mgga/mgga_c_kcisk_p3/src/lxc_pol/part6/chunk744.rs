//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 744/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk744<F: Float>(t2931: F, t852: F, t2935: F, t932: F, t3063: F, t177: F, t3042: F, t140: F, t191: F, t3043: F, t912: F, t3032: F, t919: F) -> (F, F, F, F, F) {
    let t15217 = t852 * t2931;
    let t15220 = t932 * t2935;
    let t15221 = t15220 * t3063;
    let t15224 = t3042 * t177;
    let t15226 = t140 * t15224 * t191;
    let t15232 = t912 * t3043;
    let t15237 = t3032 * t919;
    (t15217, t15221, t15226, t15232, t15237)
}
