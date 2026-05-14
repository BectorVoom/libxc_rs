//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 714/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk714<F: Float>(t5012: F, t5074: F, t99: F, t83: F, t1628: F, t496: F, t501: F, t1673: F, t1676: F, t1548: F, t546: F, t1507: F, t4913: F, t4920: F, t555: F, t12: F, t137: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t5075 = t5012 + t5074;
    let t5076 = t99 * t5075;
    let t5077 = t83 * t5076;
    let t5078 = t496 * t1628;
    let t5079 = 24.0 * t5078;
    let t5080 = t501 * t1628;
    let t5081 = 24.0 * t5080;
    let t5082 = t1673 * t1676;
    let t5086 = t1548 * t546;
    let t5087 = 96.0 * t5086;
    let t5089 = t4920 * t4913 * t1507;
    let t5091 = 0.10389515463408878255e3 * t555 * t5089;
    let t5093 = 1.0 / t137 / t12;
    (t5075, t5076, t5077, t5079, t5080, t5081, t5082, t5086, t5087, t5089, t5091, t5093)
}
