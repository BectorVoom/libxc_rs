//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 709/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk709(t5012: f64, t5074: f64, t99: f64, t83: f64, t1628: f64, t496: f64, t501: f64, t1548: f64, t546: f64, t1507: f64, t4913: f64, t4920: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5075 = t5012 + t5074;
    let t5076 = t99 * t5075;
    let t5077 = t83 * t5076;
    let t5078 = t496 * t1628;
    let t5080 = t501 * t1628;
    let t5086 = t1548 * t546;
    let t5087 = 96.0_f64 * t5086;
    let t5089 = t4920 * t4913 * t1507;
    (t5075, t5076, t5077, t5078, t5080, t5086, t5087, t5089)
}
