//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1296/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1296(t22820: f64, t3740: f64, t11234: f64, t6137: f64, t11163: f64, t881: f64, t18427: f64, t18445: f64, t18765: f64, t18766: f64, t27262: f64, t27292: f64, t27295: f64, t31067: f64, t31088: f64, t31204: f64, t31206: f64, t31208: f64, t31210: f64, t31213: f64, t31216: f64, t31218: f64, t31220: f64, t31222: f64, t31225: f64) -> (f64, f64, f64, f64) {
    let t31521 = 6.0_f64 * t22820 * t3740;
    let t31523 = 6.0_f64 * t6137 * t11234;
    let t31524 = t11163 * t881;
    let t31558 = t18765 - 0.16068111111111111111e1_f64 * t18427 + t18766 - 0.1549425e1_f64 * t27262 + 0.104195e1_f64 * t27292 + 0.20659e1_f64 * t27295 - 0.6618234375e1_f64 * t31204 + 0.794188125e1_f64 * t31206 - 0.52945875e1_f64 * t31208 - 0.52945875e1_f64 * t31210 - 0.17648625e1_f64 * t31213 + 0.2366859375e0_f64 * t31216 - 0.473371875e0_f64 * t31218 + 0.94674375e0_f64 * t31220 + 0.94674375e0_f64 * t31222 + 0.31558125e0_f64 * t31225 - 0.516475e0_f64 * t31067 + 0.1549425e1_f64 * t31088 - 0.92617777777777777776e0_f64 * t18445;
    (t31521, t31523, t31524, t31558)
}
