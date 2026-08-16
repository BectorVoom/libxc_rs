//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2313/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2313(t1022: f64, t1058: f64, t1060: f64, t1615: f64, t17686: f64, t17691: f64, t18138: f64, t23346: f64, t23613: f64, t23633: f64, t23635: f64, t23670: f64, t25429: f64, t25510: f64, t25721: f64, t28593: f64, t28618: f64, t28622: f64, t28637: f64, t28652: f64, t3186: f64, t3966: f64, t6800: f64, t7619: f64, t82625: f64, t82799: f64, t88022: f64, t89071: f64, t89176: f64) -> f64 {
    let t100377 = -0.21932454224643019154e-1_f64 * t25429 * t25510 * t89071 * t17686 + 0.73108180748810063846e-2_f64 * t25429 * t25510 * t25721 * t17691 + 0.8529287754027840782e-2_f64 * t88022 * t25510 * t89176 * t17686 - 0.36554090374405031923e-2_f64 * t25429 * t23613 * t28637 - 0.21932454224643019153e-1_f64 * t23670 * t28622 + 4.0_f64 * t3186 * t7619 * t18138 + 0.54831135561607547884e-2_f64 * t23633 * t82625 * t28652 + 0.54831135561607547884e-2_f64 * t23633 * t23635 * t3966 * t1615 * t6800 + t1058 * t28593 * t1022 * t1060 + t82799 + 0.14621636149762012769e-1_f64 * t23346 * t28618;
    t100377
}
