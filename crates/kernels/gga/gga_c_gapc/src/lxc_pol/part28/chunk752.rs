//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 752/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk752<F: Float>(t3263: F, t883: F, t3449: F, t972: F, t2712: F, t3096: F, t3430: F, t1044: F, t640: F, t916: F, t128: F, t6: F, t442: F, t919: F, t1081: F, t2645: F) -> (F, F, F, F, F, F, F, F) {
    let t9375 = t3263 * t883;
    let t9378 = t3449 * t972;
    let t9383 = t3096 * t2712;
    let t9384 = t3430 * t9383;
    let t9386 = t640 * t1044;
    let t9387 = t916 * t9386;
    let t9388 = t6 * t128;
    let t9389 = t9388 * t442;
    let t9390 = t919 * t9389;
    let t9391 = t9387 * t9390;
    let t9393 = t1081 * t2645;
    (t9375, t9378, t9384, t9386, t9387, t9388, t9391, t9393)
}
