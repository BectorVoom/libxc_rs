//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1238/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1238<F: Float>(t127: F, t56344: F, t6: F, t135: F, t161: F, t2011: F, t22856: F, t22896: F, t29750: F, t29752: F, t38749: F, t38770: F, t38783: F, t48402: F, t48487: F, t48526: F, t48555: F, t48559: F, t48571: F, t5: F, t56222: F, t56229: F, t56232: F, t56252: F, t629: F, t636: F) -> (F, F) {
    let t56346 = t6 * t56344 * t127;
    let t56350 = F::cast_from(0.16299677793353920977e0_f64) * t135 * t56229 + t2011 * t629 * t56232 / F::new(4.0) - F::cast_from(0.30426065214260652492e0_f64) * t48402 - F::new(7.0) / F::new(4.0) * t48487 + F::cast_from(0.30426065214260652492e0_f64) * t48526 + F::cast_from(0.86207184773738515393e0_f64) * t38749 + F::cast_from(0.60852130428521304982e1_f64) * t48555 - F::new(35.0) / F::new(36.0) * t38770 + F::new(5.0) / F::new(4.0) * t22896 * t629 * t5 * t56222 - F::cast_from(0.30426065214260652492e1_f64) * t48559 + F::cast_from(0.1915715217194189231e1_f64) * t29750 - F::cast_from(0.60852130428521304982e0_f64) * t48571 + F::new(455.0) / F::new(162.0) * t29752 + F::new(35.0) / F::new(12.0) * t38783 + t22856 - F::cast_from(0.10866451862235947318e-1_f64) * t135 * t56252 - F::cast_from(0.27166129655589868296e-2_f64) * t636 * t161 * t56346;
    (t56346, t56350)
}
