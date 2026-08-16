//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 566/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk566(t1664: f64, t4736: f64, t1645: f64, t1643: f64, t573: f64, t586: f64, t4705: f64, t4636: f64, t4638: f64, t4642: f64, t4646: f64, t4650: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4737 = t4736 * t1664;
    let t4739 = 1.0_f64 * t1645 * t4737;
    let t4740 = t1643 * t1643;
    let t4741 = 1.0_f64 / t4740;
    let t4742 = t573 * t4741;
    let t4743 = t586 * t586;
    let t4744 = 1.0_f64 / t4743;
    let t4745 = t4705 * t4744;
    let t4747 = 0.16081824322151104822e2_f64 * t4742 * t4745;
    let t4748 = 0.12361111111111111111e-1_f64 * t4636;
    let t4753 = t4748 + 0.61805555555555555556e-2_f64 * t4638 - 0.61805555555555555555e-2_f64 * t4642 + 0.18541666666666666667e-1_f64 * t4646 - 0.92708333333333333333e-2_f64 * t4650;
    (t4737, t4739, t4740, t4741, t4742, t4743, t4744, t4745, t4747, t4753)
}
