//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 947/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk947<F: Float>(t12013: F, t2317: F, t6525: F, t13725: F, t2312: F, t12116: F, t2321: F, t882: F, t11981: F, t2268: F, t2343: F, t6509: F) -> (F, F, F, F) {
    let t46887 = t6525 * t12013 * t2317;
    let t46889 = t2312 * t13725;
    let t46892 = t882 * t12116 * t2321;
    let t46896 = t2268 * t2343 * t11981 * t6509;
    (t46887, t46889, t46892, t46896)
}
