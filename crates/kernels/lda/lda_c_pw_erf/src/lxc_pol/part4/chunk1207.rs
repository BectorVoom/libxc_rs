//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1207/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1207<F: Float>(t34: F, t833: F, t593: F, t352: F, t1325: F, t5237: F, t6343: F, t3859: F, t6322: F, t11907: F, t12362: F, t1308: F, t1326: F, t13771: F, t13812: F, t1392: F, t13962: F, t13966: F, t1511: F, t15704: F, t15713: F, t15717: F, t15794: F, t15999: F, t16539: F, t16540: F, t184: F, t199: F, t2437: F, t2441: F, t3806: F, t3965: F, t3967: F, t3974: F, t4501: F, t4506: F, t4515: F, t4522: F, t5029: F, t5141: F, t519: F, t542: F, t571: F, t6728: F, t816: F) -> (F, F, F) {
    let t17864 = t34 * t833;
    let t17865 = t17864 * t593;
    let t17869 = t17864 * t352;
    let t17883 = t1325 * t5237 * t6343;
    let t17886 = t1325 * t3859 * t6322;
    let t17896 = -16.0 / 45.0 * t3965 * t3967 * t16539 * t542 - 32.0 / 45.0 * t3965 * t5141 * t16540 + 4.0 / 15.0 * t1511 * t2441 * t184 * t199 + 32.0 / 15.0 * t3974 * t13962 * t15717 - 32.0 / 9.0 * t3974 * t11907 * t15717 - 32.0 / 27.0 * t12362 * t4501 * t15999 - 16.0 / 15.0 * t4506 * t13966 * t15704 + 16.0 / 9.0 * t4506 * t13812 * t15704 - 64.0 / 45.0 * t13771 * t6728 * t17865 - 64.0 / 45.0 * t13771 * t4515 * t17869 + 32.0 / 27.0 * t13771 * t4522 * t17869 + 32.0 / 45.0 * t3974 * t4515 * t15713 + 8.0 / 15.0 * t519 * t1326 * t15794 + 32.0 / 81.0 * t17883 + 32.0 / 135.0 * t17886 + 8.0 / 45.0 * t519 * t3806 * t2437 * t1392 - 8.0 / 45.0 * t571 * t1308 * t816 * t5029;
    (t17864, t17865, t17896)
}
