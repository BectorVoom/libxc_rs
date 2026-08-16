//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2297/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2297(t2147: f64, t7319: f64, t11871: f64, t15032: f64, t24589: f64, t24815: f64, t24821: f64, t24859: f64, t27516: f64, t27562: f64, t3610: f64, t7387: f64, t8082: f64, t85824: f64, t85854: f64, t86037: f64, t86076: f64, t86077: f64, t94850: f64, t94947: f64, t94948: f64, t94949: f64, t94954: f64, t94963: f64, t94966: f64) -> (f64, f64) {
    let t94976 = t7319 * t2147;
    let t94980 = 0.54831135561607547884e-2_f64 * t85854 - t94947 - 0.54831135561607547884e-2_f64 * t86037 * t94948 * t94949 * t24815 + 0.27415567780803773942e-2_f64 * t86037 * t94954 * t94949 * t24821 + 0.73108180748810063846e-2_f64 * t86076 * t86077 * t94850 + 0.54831135561607547884e-2_f64 * t94963 * t85824 + 0.60923483957341719871e-3_f64 * t94966 + 0.54831135561607547884e-2_f64 * t24589 * t27516 * t24859 + 2.0_f64 * t15032 * t7387 + 2.0_f64 * t3610 * t8082 * t11871 + 0.10966227112321509577e-1_f64 * t24589 * t94976 * t27562;
    (t94976, t94980)
}
