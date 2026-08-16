//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 687/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk687(t22814: f64, t22816: f64, t1999: f64, t794: f64, t61: f64, t9222: f64, t1995: f64, t133: f64, t6933: f64, t6604: f64, t6925: f64, t242: f64, t6943: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t22817 = t22814 * t22816;
    let t22818 = t794 * t1999;
    let t22819 = t22817 * t22818;
    let t22820 = 0.16821981705891829522e-4_f64 * t22819;
    let t22822 = 1.0_f64 / t61 / t9222;
    let t22823 = t22822 * t1995;
    let t22824 = t22823 * t133;
    let t22825 = t22824 * t6933;
    let t22826 = 0.52708876011794399171e-3_f64 * t22825;
    let t22827 = t6925 * t6604;
    let t22832 = t6943 * t242;
    (t22817, t22819, t22820, t22822, t22824, t22825, t22826, t22827, t22832)
}
