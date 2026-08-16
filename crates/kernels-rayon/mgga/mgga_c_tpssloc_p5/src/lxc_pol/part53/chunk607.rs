//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 607/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk607(t234: f64, t252: f64, t776: f64, t6637: f64, t6552: f64, t1905: f64, t794: f64, t6562: f64, t6604: f64, t814: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6638 = t234 * t252;
    let t6639 = t6638 * t776;
    let t6640 = t6637 * t6639;
    let t6641 = t6552 * t6640;
    let t6643 = t794 * t1905;
    let t6644 = t6562 * t6643;
    let t6646 = t6604 * t814;
    (t6638, t6639, t6640, t6641, t6643, t6644, t6646)
}
