//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 809/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk809<F: Float>(t12013: F, t2317: F, t6525: F, t13725: F, t2312: F, t12116: F, t2321: F, t882: F, t12000: F, t555: F, t484: F, t197: F, t3689: F, t161: F, t1365: F, t38272: F) -> (F, F, F, F, F, F, F, F) {
    let t46887 = t6525 * t12013 * t2317;
    let t46889 = t2312 * t13725;
    let t46892 = t882 * t12116 * t2321;
    let t46965 = t555 * t12000;
    let t47003 = t484 * t13725;
    let t47008 = t197 * t3689;
    let t47009 = t47008 * t161;
    let t47036 = t6525 * t1365 * t38272;
    (t46887, t46889, t46892, t46965, t47003, t47008, t47009, t47036)
}
