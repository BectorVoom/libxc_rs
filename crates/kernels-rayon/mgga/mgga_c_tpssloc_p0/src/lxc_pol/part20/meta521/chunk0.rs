//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2050/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2050(t2223: f64, t3824: f64, t2475: f64, t2461: f64, t2478: f64, t159: f64, t172: f64, t2454: f64, t268: f64, t39249: f64, t39256: f64, t39300: f64, t39309: f64, t39312: f64, t39316: f64, t39320: f64, t39377: f64, t39378: f64, t39381: f64, t39535: f64, t676: f64, t724: f64, t732: f64, t739: f64, t740: f64, t746: f64, t747: f64, t781: f64, t9493: f64, t9720: f64, t9738: f64, t9740: f64, t9752: f64, t9762: f64, t9763: f64, t9781: f64, t9828: f64) -> (f64, f64, f64) {
    let t39659 = t2223 * t3824;
    let t39661 = t2475 * t2475;
    let t39664 = t2461 * t2461;
    let t39665 = t2478 * t2478;
    let t39706 = 0.19964560303604640732e6_f64 * t159 / t39661 * t39664 / t39665 - 0.14035736694323150897e2_f64 * t9762 * t39378 * t746 + t39249 + 0.91082604192152556044e5_f64 * t172 * t39377 * t39378 * t39381 - 0.12304822629859687989e5_f64 * t172 * t39535 * t39378 * t9493 + 0.5848223622634646207e0_f64 * t740 * t39300 * t746 + t39256 + t39309 - t39312 - t39316 - t39320 - 0.41096e0_f64 * t268 * t9828 * t9781 - 0.21309037037037037036e0_f64 * t268 * t781 * t724 * t732 + 0.13218100589565368422e2_f64 * t268 * t676 * t9738 * t9740 - 0.68493333333333333332e-1_f64 * t268 * t2454 * t9752 + 0.38527786510141256862e1_f64 * t268 * t676 * t9720 * t9763 - 0.67471172535210825684e-1_f64 * t268 * t781 * t739 * t747;
    (t39659, t39664, t39706)
}
