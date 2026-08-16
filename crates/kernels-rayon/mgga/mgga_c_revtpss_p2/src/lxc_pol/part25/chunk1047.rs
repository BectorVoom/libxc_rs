//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1047/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1047(t12295: f64, t12351: f64, t12297: f64, t12299: f64, t12301: f64, t12303: f64, t12307: f64, t12310: f64, t12314: f64, t12317: f64, t12320: f64, t12344: f64, t12347: f64, t12354: f64) -> f64 {
    let t12459 = 0.16068111111111111111e1_f64 * t12295;
    let t12460 = 0.46308888888888888888e0_f64 * t12351;
    let t12463 = 0.34431666666666666666e0_f64 * t12299 + 0.57386111111111111112e0_f64 * t12307 + 0.68863333333333333332e0_f64 * t12297 - 0.103295e1_f64 * t12301 - 0.51647499999999999999e0_f64 * t12303 - 0.20659e1_f64 * t12310 + 0.309885e1_f64 * t12314 + 0.516475e0_f64 * t12320 - 0.52945875e1_f64 * t12344 + 0.94674375e0_f64 * t12347 - t12459 - t12460 + 0.309885e1_f64 * t12317 + 0.6311625e0_f64 * t12354;
    t12463
}
