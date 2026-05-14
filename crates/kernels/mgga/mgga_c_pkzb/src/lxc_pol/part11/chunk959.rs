//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 959/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk959<F: Float>(t11381: F, t11416: F, t11444: F, t11481: F, t158: F, t1255: F, t3909: F, t6546: F, t3254: F, t3928: F, t11345: F, t5728: F, t6557: F, t2370: F, t3880: F, t2970: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11483 = t11381 + t11416 + t11444 + t11481;
    let t11484 = t11483 * t158;
    let t11493 = t3909 * t1255;
    let t11494 = t6546 * t11493;
    let t11497 = t3254 * t3928;
    let t11500 = t11345 * t5728;
    let t11501 = t11500 * t6557;
    let t11506 = t2370 * t3880;
    let t11507 = t2970 * t11506;
    (t11483, t11484, t11493, t11494, t11497, t11500, t11501, t11506, t11507)
}
