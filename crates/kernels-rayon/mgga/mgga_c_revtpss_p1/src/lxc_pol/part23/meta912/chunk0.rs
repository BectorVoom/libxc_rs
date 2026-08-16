//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2932/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2932(t52597: f64, t52598: f64, t63338: f64, t63340: f64, t63342: f64, t63361: f64, t63371: f64, t77539: f64, t77543: f64, t77547: f64, t77799: f64, t52128: f64, t52623: f64, t63447: f64, t63453: f64, t63459: f64, t77802: f64, t77804: f64, t77806: f64, t77810: f64, t77813: f64, t77816: f64, t77819: f64) -> (f64, f64) {
    let t77935 = -0.929655e1_f64 * t77539 + 0.309885e1_f64 * t77543 + 0.309885e1_f64 * t77547 - 0.20658999999999999999e1_f64 * t63338 + 0.68863333333333333332e0_f64 * t63340 + 0.5738611111111111111e0_f64 * t63342 + 0.309885e1_f64 * t63361 - 0.20659e1_f64 * t63371 + t52597 - t52598 + 0.6311625e0_f64 * t77799;
    let t77947 = 0.3529725e1_f64 * t77802 - 0.41678e0_f64 * t77804 + 0.69463333333333333333e-1_f64 * t77806 - t52623 + 0.92617777777777777779e0_f64 * t52128 + 0.250068e1_f64 * t77810 - 0.187551e1_f64 * t77813 + 0.62517e0_f64 * t77816 + 0.62517e0_f64 * t77819 + 0.51647499999999999999e0_f64 * t63447 - 0.45908888888888888888e0_f64 * t63453 + 0.13772666666666666667e1_f64 * t63459;
    (t77935, t77947)
}
