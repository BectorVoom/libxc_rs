//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2190/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2190(t1385: f64, t22633: f64, t22635: f64, t90516: f64, t7692: f64, t81186: f64, t26338: f64, t81228: f64, t81326: f64, t6888: f64, t7691: f64, t80707: f64) -> (f64, f64, f64, f64) {
    let t90519 = t22633 * t22635 * t90516 * t1385;
    let t90521 = t81186 * t7692;
    let t90524 = t81228 * t81326 * t26338;
    let t90525 = 0.16449340668482264365e-1_f64 * t90524;
    let t90527 = t6888 * t80707 * t7691;
    (t90519, t90521, t90525, t90527)
}
