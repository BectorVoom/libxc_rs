//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 415/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk415<F: Float>(t116: F, t3139: F, t3138: F, t979: F, t142: F, t181: F, t15: F, t163: F, t167: F, t196: F, t183: F, t816: F, t3088: F, t944: F, t151: F, t3107: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3140 = t116 * t3139;
    let t3141 = t3138 * t3140;
    let t3142 = t979 * t3141;
    let t3144 = t142 * t181;
    let t3148 = t163 * t15;
    let t3155 = t196 * t167;
    let t3156 = t816 * t183;
    let t3162 = t944 * t3088;
    let t3166 = t151 * t3107;
    (t3140, t3141, t3142, t3144, t3148, t3155, t3156, t3162, t3166)
}
