//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 859/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk859<F: Float>(t2935: F, t932: F, t3063: F, t177: F, t3042: F, t140: F, t191: F, t3043: F, t912: F, t3032: F, t919: F, t116: F, t5821: F, t114: F, t923: F, t3052: F, t927: F) -> (F, F, F, F, F, F, F, F, F) {
    let t15220 = t932 * t2935;
    let t15221 = t15220 * t3063;
    let t15224 = t3042 * t177;
    let t15226 = t140 * t15224 * t191;
    let t15232 = t912 * t3043;
    let t15237 = t3032 * t919;
    let t15244 = t116 * t5821;
    let t15245 = t114 * t15244;
    let t15250 = t923 * t923;
    let t15251 = 1.0 / t15250;
    let t15253 = t3052 * t927;
    (t15220, t15221, t15224, t15226, t15232, t15237, t15245, t15251, t15253)
}
