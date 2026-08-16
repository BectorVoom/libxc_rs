//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 533/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk533(t423: f64, t4737: f64, t1098: f64, t1657: f64, t1119: f64, t1671: f64, t3259: f64, t1117: f64, t3264: f64, t1661: f64, t3270: f64, t1102: f64) -> (f64, f64, f64, f64, f64) {
    let t4739 = 0.621814e-1_f64 * t4737 * t423;
    let t4740 = t1657 * t1098;
    let t4742 = 1.0_f64 * t4740 * t1119;
    let t4744 = 1.0_f64 * t3259 * t1671;
    let t4745 = t1671 * t1117;
    let t4747 = 2.0_f64 * t3264 * t4745;
    let t4748 = t3270 * t1661;
    let t4749 = t4748 * t1102;
    (t4739, t4742, t4744, t4747, t4749)
}
