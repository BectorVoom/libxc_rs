//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2077/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2077(t90524: f64, t22892: f64, t7691: f64, t80645: f64, t26206: f64, t6883: f64, t1834: f64, t794: f64, t6891: f64, t22704: f64, t26355: f64, t81326: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t90525 = 0.16449340668482264365e-1_f64 * t90524;
    let t90533 = t22892 * t80645 * t7691;
    let t90534 = 0.16449340668482264365e-1_f64 * t90533;
    let t90541 = t6883 * t26206;
    let t90542 = 0.38381794893125283518e-1_f64 * t90541;
    let t90544 = t794 * t1834;
    let t90546 = t22892 * t90544 * t6891;
    let t90547 = 0.16449340668482264365e-1_f64 * t90546;
    let t90549 = t22704 * t81326 * t26355;
    (t90525, t90534, t90542, t90544, t90547, t90549)
}
