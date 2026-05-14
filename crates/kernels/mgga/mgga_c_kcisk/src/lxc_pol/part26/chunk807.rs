//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 807/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk807<F: Float>(t12699: F, t214: F, t12698: F, t1035: F, t9352: F, t3127: F, t981: F, t1045: F, t3236: F, t3248: F, t3132: F, t3263: F, t12697: F, t205: F, t207: F, t1050: F, t3139: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12700 = t214 * t12699;
    let t12701 = t12698 * t12700;
    let t12703 = t1035 * t9352;
    let t12705 = t3127 * t981;
    let t12706 = t12705 * t1045;
    let t12708 = t3236 * t3248;
    let t12710 = t3132 * t3263;
    let t12712 = t205 * t12697;
    let t12713 = t207 * t12699;
    let t12714 = t12712 * t12713;
    let t12716 = t1050 * t3139;
    (t12701, t12703, t12705, t12706, t12708, t12710, t12712, t12714, t12716)
}
