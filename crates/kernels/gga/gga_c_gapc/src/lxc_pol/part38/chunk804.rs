//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 804/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk804<F: Float>(t442: F, t9388: F, t919: F, t9387: F, t1081: F, t2645: F, t7451: F, t8673: F, t6182: F, t8676: F, t1084: F, t8906: F) -> (F, F, F, F, F) {
    let t9389 = t9388 * t442;
    let t9390 = t919 * t9389;
    let t9391 = t9387 * t9390;
    let t9393 = t1081 * t2645;
    let t9395 = t7451 * t8673;
    let t9396 = t8676 * t6182;
    let t9397 = t9395 * t9396;
    let t9399 = t1084 * t8906;
    (t9391, t9393, t9396, t9397, t9399)
}
