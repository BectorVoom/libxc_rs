//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1210/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1210(t2832: f64, t890: f64, t2430: f64, t1100: f64, t3329: f64, t10259: f64, t93: f64, t2037: f64, t4168: f64, t1455: f64, t7337: f64, t2045: f64, t4153: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t51792 = t890 * t2832;
    let t51806 = t2430 * t890;
    let t52188 = t1100 * t3329;
    let t60551 = t93 * t10259;
    let t92556 = t2037 * t4168;
    let t92559 = t1455 * t7337;
    let t92563 = t4153 * t2045;
    (t51792, t51806, t52188, t60551, t92556, t92559, t92563)
}
