//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 905/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk905<F: Float>(t11633: F, t531: F, t10338: F, t1474: F, t1444: F, t461: F, t543: F, t1479: F, t3251: F, t1484: F, t11402: F, t513: F, t1416: F, t3820: F, t11407: F, t2820: F, t3751: F, t86: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11634 = t11633 * t531;
    let t11640 = t10338 * t1474;
    let t11670 = 1.0 / t461 / t1444;
    let t11671 = t11670 * t543;
    let t11721 = t3251 * t1479;
    let t11723 = t3251 * t1484;
    let t11727 = t11402 * t513;
    let t11730 = t3820 * t1416;
    let t11746 = 0.12841111111111111111e-1 * t11407;
    let t11814 = t86 * t2820 * t3751;
    (t11634, t11640, t11670, t11671, t11721, t11723, t11727, t11730, t11746, t11814)
}
