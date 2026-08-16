//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1047/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1047<F: Float>(t127: F, t22769: F, t6: F, t135: F, t161: F, t2021: F, t22219: F, t22222: F, t22225: F, t22228: F, t22230: F, t22233: F, t22236: F, t22238: F, t22240: F, t22243: F, t22247: F, t22253: F, t22257: F, t22261: F, t22263: F, t22266: F, t636: F) -> (F, F) {
    let t22771 = t6 * t22769 * t127;
    let t22775 = F::cast_from(0.65198711173415683908e0_f64) * t2021 * t22219 - F::cast_from(0.60852130428521304982e0_f64) * t22222 + F::cast_from(0.65198711173415683908e-1_f64) * t636 * t22225 - F::cast_from(0.17241436954747703079e1_f64) * t22228 + F::cast_from(0.20284043476173768328e0_f64) * t22230 - F::cast_from(0.10866451862235947318e-1_f64) * t135 * t22233 + F::cast_from(0.76628608687767569239e1_f64) * t22236 + F::cast_from(0.5071010869043442082e-1_f64) * t22238 - F::cast_from(0.43103592386869257697e0_f64) * t22240 + F::cast_from(0.86207184773738515393e0_f64) * t22243 - F::cast_from(0.27166129655589868296e-2_f64) * t636 * t161 * t22247 + F::cast_from(0.16299677793353920977e-1_f64) * t2021 * t161 * t22253 - F::cast_from(0.81498388966769604888e-2_f64) * t636 * t161 * t22257 + F::cast_from(0.5071010869043442082e-1_f64) * t22261 - F::cast_from(0.30426065214260652492e0_f64) * t22263 + F::cast_from(0.30426065214260652492e0_f64) * t22266 - F::cast_from(0.27166129655589868296e-2_f64) * t636 * t161 * t22771;
    (t22771, t22775)
}
