//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1222/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1222<F: Float>(t4804: F, t6265: F, t3794: F, t504: F, t6590: F, t1325: F, t1326: F, t348: F, t108: F, t181: F, t16143: F, t198: F, t266: F, t13922: F, t13924: F, t13926: F) -> (F, F, F, F, F, F, F) {
    let t18130 = 16.0 / 45.0 * t4804 * t6265;
    let t18132 = 16.0 / 45.0 * t3794 * t6265;
    let t18133 = t6590 * t504;
    let t18137 = 16.0 / 45.0 * t1325 * t1326 * t18133 * t348;
    let t18138 = t181 * t108;
    let t18142 = 8.0 / 15.0 * t16143 * t18138 * t266 * t198;
    let t18143 = 32.0 / 135.0 * t13922;
    let t18144 = 32.0 / 135.0 * t13924;
    let t18145 = 32.0 / 45.0 * t13926;
    (t18130, t18132, t18137, t18142, t18143, t18144, t18145)
}
