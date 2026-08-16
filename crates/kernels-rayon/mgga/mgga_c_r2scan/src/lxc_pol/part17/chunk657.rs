//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 657/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk657(t4741: f64, t1736: f64, t3: f64, t40: f64, t1401: f64, t4735: f64, t392: f64, t4738: f64, t22: f64, t502: f64, t6: f64, t4733: f64, t4736: f64, t4739: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4742 = 0.16068111111111111111e1_f64 * t4741;
    let t4743 = t1736 * t3;
    let t4744 = t4743 * t40;
    let t4745 = 0.28051666666666666666e0_f64 * t4744;
    let t4746 = t1401 * t4735;
    let t4747 = 0.56103333333333333332e0_f64 * t4746;
    let t4748 = t392 * t4738;
    let t4749 = 0.6545388888888888889e0_f64 * t4748;
    let t4751 = t22 * t6 * t502;
    let t4752 = 0.46308888888888888888e0_f64 * t4751;
    let t4753 = -0.47063e1_f64 * t4733 + 0.31375333333333333334e1_f64 * t4736 - 0.36604555555555555556e1_f64 * t4739 - t4742 + t4745 - t4747 - t4749 - t4752;
    (t4742, t4744, t4745, t4746, t4747, t4748, t4749, t4751, t4752, t4753)
}
