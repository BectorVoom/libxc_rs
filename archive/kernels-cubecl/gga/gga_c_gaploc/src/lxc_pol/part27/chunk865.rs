//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 865/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk865<F: Float>(t701: F, t8502: F, t6066: F, t326: F, t7573: F, t1991: F, t2004: F, t2033: F, t2070: F, t2615: F, t2976: F, t2979: F, t3002: F, t3006: F, t3040: F, t3043: F, t5577: F, t5666: F, t5669: F, t5676: F, t5694: F, t5703: F, t7563: F, t7565: F, t7572: F, t7630: F, t780: F, t8530: F, t8535: F, t8540: F, t8550: F, t8553: F, t8556: F) -> (F, F) {
    let t8561 = t8502 * t701;
    let t8562 = t6066 * t8561;
    let t8565 = t326 * t8561;
    let t8568 = t7573 * t8561;
    let t8573 = F::cast_from(0.79445533226334281486e-1_f64) * t5676 * t2979 + F::cast_from(0.79445533226334281486e-1_f64) * t2033 * t8530 + F::cast_from(0.92686455430723328401e-1_f64) * t2976 * t5694 - F::cast_from(0.92686455430723328401e-1_f64) * t2033 * t8535 + F::cast_from(0.1022478025437886658e1_f64) * t5669 * t3002 + F::cast_from(0.1022478025437886658e1_f64) * t1991 * t8540 - F::cast_from(0.1022478025437886658e1_f64) * t5577 * t3006 + F::cast_from(0.51123901271894332905e0_f64) * t5666 * t3006 + F::cast_from(0.71500979903700853338e0_f64) * t5703 * t3043 + F::cast_from(0.71500979903700853338e0_f64) * t2004 * t8550 + F::cast_from(0.35750489951850426669e0_f64) * t2004 * t8553 + F::cast_from(0.47667319935800568892e0_f64) * t780 * t8556 + F::cast_from(0.71500979903700853338e0_f64) * t2070 * t3040 - F::cast_from(0.14300195980740170668e1_f64) * t7630 * t8562 + F::cast_from(0.92023022289409799224e1_f64) * t2615 * t8565 + F::cast_from(0.13803453343411469884e2_f64) * t7572 * t8568 - F::cast_from(0.11916829983950142223e0_f64) * t7563 + F::cast_from(0.59584149919750711116e-1_f64) * t7565;
    (t8561, t8573)
}
