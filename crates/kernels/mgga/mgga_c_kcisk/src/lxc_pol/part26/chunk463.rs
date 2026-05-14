//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 463/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk463<F: Float>(t196: F, t2925: F, t179: F, t852: F, t15: F, t60: F, t989: F, t816: F, t183: F, t20: F, t21: F, t963: F, t151: F, t1014: F, t142: F, t955: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3190 = t2925 * t196;
    let t3193 = t852 * t179;
    let t3194 = t3193 * t15;
    let t3199 = t60 * t989;
    let t3200 = t3199 * t816;
    let t3201 = t183 * t20;
    let t3203 = t3201 * t21 * t963;
    let t3206 = t15 * t151;
    let t3207 = t1014 * t3206;
    let t3208 = t142 * t955;
    (t3190, t3193, t3194, t3199, t3200, t3201, t3203, t3206, t3207, t3208)
}
