//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1037/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1037<F: Float>(t11500: F, t6557: F, t2370: F, t3880: F, t2970: F, t6570: F, t11369: F, t133: F, t945: F, t1227: F, t394: F, t6591: F) -> (F, F, F, F, F, F, F, F) {
    let t11501 = t11500 * t6557;
    let t11506 = t2370 * t3880;
    let t11507 = t2970 * t11506;
    let t11510 = t11500 * t6570;
    let t11519 = t11369 * t133;
    let t11520 = t11519 * t945;
    let t11524 = t2970 * t1227 * t394;
    let t11527 = t11500 * t6591;
    (t11501, t11506, t11507, t11510, t11519, t11520, t11524, t11527)
}
