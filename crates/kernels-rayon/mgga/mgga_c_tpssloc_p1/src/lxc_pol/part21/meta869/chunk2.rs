//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3184/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3184(t1227: f64, t13969: f64, t18589: f64, t15743: f64, t5005: f64, t1174: f64, t6177: f64, t698: f64, t11709: f64, t15455: f64, t15459: f64, t15463: f64, t15525: f64, t15535: f64, t15569: f64, t15612: f64, t15631: f64, t15650: f64, t1653: f64, t18321: f64, t19058: f64, t3552: f64, t3557: f64, t3560: f64, t3577: f64, t3578: f64, t5024: f64, t52906: f64, t53083: f64, t53087: f64, t55723: f64, t974: f64) -> f64 {
    let t66052 = t1227 * t13969 * t18589;
    let t66054 = t5005 * t15743;
    let t66057 = t1174 * t698 * t6177;
    let t66067 = 5.0_f64 / 486.0_f64 * t5024 * t15455 + t53083 * t15631 / 48.0_f64 - t53087 * t15535 / 288.0_f64 + t11709 * t19058 / 768.0_f64 + t5024 * t15650 / 108.0_f64 + t5024 * t15612 / 216.0_f64 - t3577 * t3578 * t15525 * t1653 / 2304.0_f64 + t15569 * t15459 / 432.0_f64 + t15569 * t15463 / 216.0_f64 - t52906 / 216.0_f64 - t66052 / 864.0_f64 + 5.0_f64 / 5184.0_f64 * t66054 - t66057 / 972.0_f64 + t1174 * t974 * t3560 * t55723 / 108.0_f64 - 11.0_f64 / 324.0_f64 * t18321 * t3552 - 11.0_f64 / 162.0_f64 * t18321 * t3557;
    t66067
}
