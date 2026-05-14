//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1068/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1068<F: Float>(t3262: F, t3263: F, t40374: F, t114: F, t1543: F, t97: F, t3575: F, t481: F, t7040: F, t3446: F, t3453: F, t7098: F, t7101: F, t104: F, t920: F, t36988: F) -> (F, F, F, F, F, F) {
    let t40377 = 3.0 / 4.0 * t3262 * t3263 * t40374;
    let t40379 = t97 * t1543 * t114;
    let t40381 = 3.0 / 2.0 * t40379 * t3575;
    let t40383 = t7040 * t481;
    let t40386 = 3.0 / 2.0 * t3262 * t3263 * t40383;
    let t40388 = t3446 * t3453 * t7098;
    let t40391 = t3446 * t3453 * t7101;
    let t40393 = t104 * t920;
    let t40394 = t97 * t40393;
    let t40396 = 3.0 / 2.0 * t40394 * t36988;
    (t40377, t40381, t40386, t40388, t40391, t40396)
}
