//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 997/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk997<F: Float>(t158: F, t2461: F, t3446: F, t3447: F, t874: F, t122: F, t3434: F, t3437: F, t3348: F, t983: F, t11002: F, t10831: F, t1102: F, t3692: F, t2333: F, t2526: F) -> (F, F, F, F, F) {
    let t40453 = t158 * t2461;
    let t40456 = t3446 * t3447 * t40453 * t874;
    let t40457 = 0.30487649791575028314e-3 * t40456;
    let t40460 = t3434 * t3437 * t40453 * t122;
    let t40461 = 0.43368970657079495312e-4 * t40460;
    let t40472 = t3348 * t983;
    let t40473 = t11002 * t40472;
    let t40485 = t1102 * t10831 * t3692;
    let t40491 = t2333 * t2526;
    (t40457, t40461, t40473, t40485, t40491)
}
