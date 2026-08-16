//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 829/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk829(t1671: f64, t3259: f64, t1117: f64, t3264: f64, t1661: f64, t3270: f64, t1102: f64, t3238: f64, t3274: f64, t4721: f64, t4726: f64, t4731: f64, t4735: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4744 = 1.0_f64 * t3259 * t1671;
    let t4745 = t1671 * t1117;
    let t4747 = 2.0_f64 * t3264 * t4745;
    let t4748 = t3270 * t1661;
    let t4749 = t4748 * t1102;
    let t4756 = t3274 - t3238 / 9.0_f64 - t4721 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t4726 + 2.0_f64 / 3.0_f64 * t4731 + t4735 / 3.0_f64;
    (t4744, t4745, t4747, t4748, t4749, t4756)
}
