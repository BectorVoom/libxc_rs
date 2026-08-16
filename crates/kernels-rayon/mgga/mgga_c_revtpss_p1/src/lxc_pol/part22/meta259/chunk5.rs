//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1600/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1600(t300: f64, t6541: f64, t6514: f64, t1765: f64, t5192: f64, t1188: f64, t3495: f64, t6518: f64, t1196: f64, t1179: f64, t6534: f64, t3520: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6542 = t300 * t6541;
    let t6544 = 0.19751673498613801407e-1_f64 * t300 * t6514;
    let t6546 = 0.11696447245269292414e1_f64 * t5192 * t1765;
    let t6548 = t3495 * t6518 * t1188;
    let t6550 = 0.11696447245269292414e1_f64 * t1196 * t6548;
    let t6552 = t1179 * t6534 * t1188;
    let t6554 = 0.5848223622634646207e0_f64 * t1196 * t6552;
    let t6555 = t3520 * t6518;
    (t6542, t6544, t6546, t6548, t6550, t6552, t6554, t6555)
}
