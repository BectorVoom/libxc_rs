//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1047/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1047(t127: f64, t22769: f64, t6: f64, t135: f64, t161: f64, t2021: f64, t22219: f64, t22222: f64, t22225: f64, t22228: f64, t22230: f64, t22233: f64, t22236: f64, t22238: f64, t22240: f64, t22243: f64, t22247: f64, t22253: f64, t22257: f64, t22261: f64, t22263: f64, t22266: f64, t636: f64) -> (f64, f64) {
    let t22771 = t6 * t22769 * t127;
    let t22775 = 0.65198711173415683908e0_f64 * t2021 * t22219 - 0.60852130428521304982e0_f64 * t22222 + 0.65198711173415683908e-1_f64 * t636 * t22225 - 0.17241436954747703079e1_f64 * t22228 + 0.20284043476173768328e0_f64 * t22230 - 0.10866451862235947318e-1_f64 * t135 * t22233 + 0.76628608687767569239e1_f64 * t22236 + 0.5071010869043442082e-1_f64 * t22238 - 0.43103592386869257697e0_f64 * t22240 + 0.86207184773738515393e0_f64 * t22243 - 0.27166129655589868296e-2_f64 * t636 * t161 * t22247 + 0.16299677793353920977e-1_f64 * t2021 * t161 * t22253 - 0.81498388966769604888e-2_f64 * t636 * t161 * t22257 + 0.5071010869043442082e-1_f64 * t22261 - 0.30426065214260652492e0_f64 * t22263 + 0.30426065214260652492e0_f64 * t22266 - 0.27166129655589868296e-2_f64 * t636 * t161 * t22771;
    (t22771, t22775)
}
