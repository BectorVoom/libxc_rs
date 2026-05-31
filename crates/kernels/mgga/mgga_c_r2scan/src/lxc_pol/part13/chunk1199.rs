//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1199/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1199<F: Float>(t3262: F, t3263: F, t40383: F, t3446: F, t3453: F, t7098: F, t7101: F, t104: F, t920: F, t97: F, t36988: F, t1234: F, t3582: F) -> (F, F, F, F, F) {
    let t40386 = F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t3262 * t3263 * t40383;
    let t40388 = t3446 * t3453 * t7098;
    let t40391 = t3446 * t3453 * t7101;
    let t40393 = t104 * t920;
    let t40394 = t97 * t40393;
    let t40396 = F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t40394 * t36988;
    let t40397 = t3582 * t1234;
    (t40386, t40388, t40391, t40396, t40397)
}
