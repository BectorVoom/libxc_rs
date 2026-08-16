//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1238/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1238(t127: f64, t56344: f64, t6: f64, t135: f64, t161: f64, t2011: f64, t22856: f64, t22896: f64, t29750: f64, t29752: f64, t38749: f64, t38770: f64, t38783: f64, t48402: f64, t48487: f64, t48526: f64, t48555: f64, t48559: f64, t48571: f64, t5: f64, t56222: f64, t56229: f64, t56232: f64, t56252: f64, t629: f64, t636: f64) -> (f64, f64) {
    let t56346 = t6 * t56344 * t127;
    let t56350 = 0.16299677793353920977e0_f64 * t135 * t56229 + t2011 * t629 * t56232 / 4.0_f64 - 0.30426065214260652492e0_f64 * t48402 - 7.0_f64 / 4.0_f64 * t48487 + 0.30426065214260652492e0_f64 * t48526 + 0.86207184773738515393e0_f64 * t38749 + 0.60852130428521304982e1_f64 * t48555 - 35.0_f64 / 36.0_f64 * t38770 + 5.0_f64 / 4.0_f64 * t22896 * t629 * t5 * t56222 - 0.30426065214260652492e1_f64 * t48559 + 0.1915715217194189231e1_f64 * t29750 - 0.60852130428521304982e0_f64 * t48571 + 455.0_f64 / 162.0_f64 * t29752 + 35.0_f64 / 12.0_f64 * t38783 + t22856 - 0.10866451862235947318e-1_f64 * t135 * t56252 - 0.27166129655589868296e-2_f64 * t636 * t161 * t56346;
    (t56346, t56350)
}
