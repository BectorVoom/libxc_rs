//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 656/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk656<F: Float>(t1031: F, t3137: F, t3242: F, t980: F, t177: F, t1001: F, t3139: F, t214: F, t1035: F, t9352: F, t3127: F, t981: F, t1045: F, t3236: F, t3248: F, t3132: F, t3263: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t12693 = t1031 * t3137;
    let t12694 = t12693 * t3242;
    let t12696 = t980 * t980;
    let t12697 = 1.0 / t12696;
    let t12698 = t177 * t12697;
    let t12699 = t3139 * t1001;
    let t12700 = t214 * t12699;
    let t12701 = t12698 * t12700;
    let t12703 = t1035 * t9352;
    let t12705 = t3127 * t981;
    let t12706 = t12705 * t1045;
    let t12708 = t3236 * t3248;
    let t12710 = t3132 * t3263;
    (t12694, t12697, t12698, t12699, t12701, t12703, t12705, t12706, t12708, t12710)
}
