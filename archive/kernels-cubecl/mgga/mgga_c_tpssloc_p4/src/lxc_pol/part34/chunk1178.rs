//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1178/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1178<F: Float>(t22986: F, t25249: F, t5612: F, t6646: F, t1510: F, t98389: F, t98422: F, t20756: F, t6637: F, t6638: F, t81984: F, t1888: F, t22996: F, t2632: F, t67358: F) -> (F, F, F, F, F) {
    let t105661 = t22986 * t6646 * t25249 * t5612;
    let t105665 = t22986 * t6646 * t98389 * t1510;
    let t105669 = t22986 * t6646 * t98422 * t1510;
    let t105674 = t81984 * t6637 * t6638 * t20756;
    let t105685 = t1888 * t22996 * t67358 * t2632;
    (t105661, t105665, t105669, t105674, t105685)
}
