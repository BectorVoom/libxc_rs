//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2061/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2061(t90514: f64, t1377: f64, t5187: f64, t7692: f64, t81186: f64, t26338: f64, t81228: f64, t81326: f64, t22892: f64, t7691: f64, t80645: f64, t26206: f64, t6883: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t90515 = 0.82246703342411321824e-2_f64 * t90514;
    let t90516 = t1377 * t5187;
    let t90521 = t81186 * t7692;
    let t90524 = t81228 * t81326 * t26338;
    let t90525 = 0.16449340668482264365e-1_f64 * t90524;
    let t90533 = t22892 * t80645 * t7691;
    let t90534 = 0.16449340668482264365e-1_f64 * t90533;
    let t90541 = t6883 * t26206;
    (t90515, t90516, t90521, t90525, t90534, t90541)
}
