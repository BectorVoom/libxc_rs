//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1031/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1031<F: Float>(t1551: F, t2562: F, t360: F, t2526: F, t537: F, t2124: F, t495: F, t2719: F, t277: F) -> (F, F, F, F, F) {
    let t7990 = t2562 * t1551;
    let t7991 = t360 * t7990;
    let t7994 = t537 * t2526;
    let t7996 = t2124 * t7994 * t495;
    let t8001 = t277 * t2719;
    (t7990, t7991, t7994, t7996, t8001)
}
