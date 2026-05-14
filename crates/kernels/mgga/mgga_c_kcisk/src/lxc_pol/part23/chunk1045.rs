//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1045/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1045<F: Float>(t20964: F, t6316: F, t4197: F, t6388: F, t1500: F, t4229: F, t6323: F, t19904: F, t6322: F, t6321: F, t2262: F, t4312: F, t1487: F, t4189: F, t6382: F, t2274: F, t4181: F) -> (F, F, F, F, F, F, F, F) {
    let t20965 = t6316 * t20964;
    let t20967 = t6388 * t4197;
    let t20969 = t1500 * t4229;
    let t20970 = t20969 * t6323;
    let t20972 = t6322 * t19904;
    let t20973 = t6321 * t20972;
    let t20975 = t4312 * t2262;
    let t20976 = t1487 * t20975;
    let t20978 = t6382 * t4189;
    let t20980 = t4181 * t2274;
    (t20965, t20967, t20970, t20972, t20973, t20976, t20978, t20980)
}
