//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 960/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk960<F: Float>(t11500: F, t6570: F, t11369: F, t133: F, t945: F, t1227: F, t2970: F, t394: F, t6591: F, t10344: F, t10356: F, t11483: F, t11501: F, t11507: F, t1250: F, t3259: F, t3273: F, t3914: F, t3920: F, t3923: F, t397: F, t6555: F, t6569: F, t6590: F, t8546: F, t8554: F, t943: F) -> (F, F, F, F, F, F) {
    let t11510 = t11500 * t6570;
    let t11519 = t11369 * t133;
    let t11520 = t11519 * t945;
    let t11524 = t2970 * t1227 * t394;
    let t11527 = t11500 * t6591;
    let t11532 = 0.39512695097613069591e1 * t6555 * t11501 + 0.39512695097613069591e1 * t8546 * t3914 + 0.39512695097613069591e1 * t3259 * t11507 - 0.39512695097613069591e1 * t6569 * t11510 + 0.19756347548806534796e1 * t10356 * t1250 + 0.19756347548806534796e1 * t3273 * t3920 - 0.19756347548806534796e1 * t8554 * t3923 + 0.65854491829355115987e0 * t943 * t11520 - 0.19756347548806534796e1 * t10344 * t11524 + 0.65854491829355115987e0 * t6590 * t11527 + 0.65854491829355115987e0 * t397 * t11483;
    (t11510, t11519, t11520, t11524, t11527, t11532)
}
