//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 865/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk865(t701: f64, t8502: f64, t6066: f64, t326: f64, t7573: f64, t1991: f64, t2004: f64, t2033: f64, t2070: f64, t2615: f64, t2976: f64, t2979: f64, t3002: f64, t3006: f64, t3040: f64, t3043: f64, t5577: f64, t5666: f64, t5669: f64, t5676: f64, t5694: f64, t5703: f64, t7563: f64, t7565: f64, t7572: f64, t7630: f64, t780: f64, t8530: f64, t8535: f64, t8540: f64, t8550: f64, t8553: f64, t8556: f64) -> (f64, f64) {
    let t8561 = t8502 * t701;
    let t8562 = t6066 * t8561;
    let t8565 = t326 * t8561;
    let t8568 = t7573 * t8561;
    let t8573 = 0.79445533226334281486e-1_f64 * t5676 * t2979 + 0.79445533226334281486e-1_f64 * t2033 * t8530 + 0.92686455430723328401e-1_f64 * t2976 * t5694 - 0.92686455430723328401e-1_f64 * t2033 * t8535 + 0.1022478025437886658e1_f64 * t5669 * t3002 + 0.1022478025437886658e1_f64 * t1991 * t8540 - 0.1022478025437886658e1_f64 * t5577 * t3006 + 0.51123901271894332905e0_f64 * t5666 * t3006 + 0.71500979903700853338e0_f64 * t5703 * t3043 + 0.71500979903700853338e0_f64 * t2004 * t8550 + 0.35750489951850426669e0_f64 * t2004 * t8553 + 0.47667319935800568892e0_f64 * t780 * t8556 + 0.71500979903700853338e0_f64 * t2070 * t3040 - 0.14300195980740170668e1_f64 * t7630 * t8562 + 0.92023022289409799224e1_f64 * t2615 * t8565 + 0.13803453343411469884e2_f64 * t7572 * t8568 - 0.11916829983950142223e0_f64 * t7563 + 0.59584149919750711116e-1_f64 * t7565;
    (t8561, t8573)
}
