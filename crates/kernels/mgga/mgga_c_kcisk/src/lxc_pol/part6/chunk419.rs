//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 419/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk419<F: Float>(t183: F, t20: F, t21: F, t963: F, t15: F, t151: F, t1014: F, t142: F, t955: F, t5: F, t1016: F, t119: F, t4: F, t181: F, t944: F, t3088: F) -> (F, F, F, F, F, F, F, F) {
    let t3201 = t183 * t20;
    let t3203 = t3201 * t21 * t963;
    let t3206 = t15 * t151;
    let t3207 = t1014 * t3206;
    let t3208 = t142 * t955;
    let t3209 = t5 * t3208;
    let t3213 = t1016 * t4 * t119;
    let t3216 = t181 * t944;
    let t3217 = t3216 * t3088;
    (t3201, t3203, t3206, t3207, t3209, t3213, t3216, t3217)
}
