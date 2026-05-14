//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1049/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1049<F: Float>(t113: F, t3578: F, t97: F, t11056: F, t2378: F, t2381: F, t37028: F, t1010: F, t1276: F, t11053: F, t8358: F, t19141: F, t3629: F, t11888: F, t6654: F, t2391: F, t3366: F) -> (F, F, F, F, F, F, F, F) {
    let t40713 = t97 * t3578 * t113;
    let t40779 = t2378 * t11056;
    let t40781 = t37028 * t2381;
    let t40788 = t1276 * t11056 * t1010;
    let t40790 = t8358 * t11053;
    let t40792 = t19141 * t3629;
    let t40794 = t6654 * t11888;
    let t40797 = t1276 * t3366 * t2391;
    (t40713, t40779, t40781, t40788, t40790, t40792, t40794, t40797)
}
