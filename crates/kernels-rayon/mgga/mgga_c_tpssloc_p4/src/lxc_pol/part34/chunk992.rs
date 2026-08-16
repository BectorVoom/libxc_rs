//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 992/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk992(t22814: f64, t22816: f64, t1999: f64, t794: f64, t61: f64, t9222: f64, t1995: f64, t133: f64, t6933: f64, t6604: f64, t6925: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22817 = t22814 * t22816;
    let t22818 = t794 * t1999;
    let t22819 = t22817 * t22818;
    let t22822 = 1.0_f64 / t61 / t9222;
    let t22823 = t22822 * t1995;
    let t22824 = t22823 * t133;
    let t22825 = t22824 * t6933;
    let t22827 = t6925 * t6604;
    (t22818, t22819, t22822, t22823, t22825, t22827)
}
