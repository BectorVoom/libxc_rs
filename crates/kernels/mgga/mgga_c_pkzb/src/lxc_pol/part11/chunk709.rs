//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 709/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk709<F: Float>(t5012: F, t5074: F, t99: F, t83: F, t1628: F, t496: F, t501: F, t1548: F, t546: F, t1507: F, t4913: F, t4920: F) -> (F, F, F, F, F, F, F, F) {
    let t5075 = t5012 + t5074;
    let t5076 = t99 * t5075;
    let t5077 = t83 * t5076;
    let t5078 = t496 * t1628;
    let t5080 = t501 * t1628;
    let t5086 = t1548 * t546;
    let t5087 = F::new(96.0) * t5086;
    let t5089 = t4920 * t4913 * t1507;
    (t5075, t5076, t5077, t5078, t5080, t5086, t5087, t5089)
}
