//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 552/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk552<F: Float>(t416: F, t8161: F, t467: F, t471: F, t415: F, t2059: F, t2231: F, t3797: F, t3796: F, t3482: F, t2152: F, t3485: F, t3484: F, t1341: F, t7740: F, t1340: F) -> (F, F, F, F, F, F, F, F) {
    let t8162 = t416 * t8161;
    let t8163 = t8162 * t467;
    let t8164 = t8163 * t471;
    let t8165 = t415 * t8164;
    let t8170 = t2059 * t2231;
    let t8171 = t3797 * t8170;
    let t8172 = t3796 * t8171;
    let t8173 = t3482 * t8172;
    let t8175 = t2059 * t2152;
    let t8176 = t3485 * t8175;
    let t8177 = t3484 * t8176;
    let t8178 = t3482 * t8177;
    let t8180 = t1341 * t7740;
    let t8181 = t1340 * t8180;
    (t8164, t8165, t8172, t8173, t8177, t8178, t8180, t8181)
}
