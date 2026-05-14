//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1114/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1114<F: Float>(t2139: F, t2294: F, t6575: F, t2598: F, t6144: F, t6583: F, t6585: F, t6129: F, t7494: F, t277: F, t5065: F, t489: F, t6188: F, t6189: F, t2224: F, t625: F) -> (F, F, F, F, F, F, F) {
    let t20052 = t2139 * t2294 * t6575;
    let t20059 = t2598 * t2294 * t6144;
    let t20080 = t6583 * t2294 * t6585;
    let t20082 = t7494 * t6129;
    let t20084 = t277 * t5065;
    let t20090 = t6188 * t6189 * t489;
    let t20092 = t20090 * t2224 * t625;
    (t20052, t20059, t20080, t20082, t20084, t20090, t20092)
}
