//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1319/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1319(t10472: f64, t10475: f64, t42559: f64, t3128: f64, t10903: f64, t10948: f64, t10890: f64, t10898: f64, t3103: f64, t1000: f64, t10390: f64, t10405: f64, t10410: f64, t10415: f64, t10485: f64, t10860: f64, t10879: f64, t10919: f64, t3043: f64, t3109: f64, t3117: f64, t3123: f64, t3134: f64, t42541: f64, t42546: f64, t42552: f64, t42554: f64, t42557: f64) -> f64 {
    let t42561 = t10472 * t10475 * t42559;
    let t42565 = t10472 * t3128 * t42559;
    let t42570 = t10948 * t10903;
    let t42573 = t10948 * t10890;
    let t42578 = t10898 * t3103;
    let t42580 = t42541 * t10405 / 192.0_f64 + 5.0_f64 / 1152.0_f64 * t10390 * t10410 - t42546 * t10415 / 384.0_f64 + 5.0_f64 / 1152.0_f64 * t3117 * t10919 + 5.0_f64 / 972.0_f64 * t42552 - 154.0_f64 / 243.0_f64 * t42554 * t1000 + 11.0_f64 / 81.0_f64 * t42557 - t42561 * t10485 / 24.0_f64 + t42565 * t10879 / 24.0_f64 - t10898 * t3123 / 48.0_f64 - t42570 * t3134 / 24.0_f64 + t42573 * t3043 / 48.0_f64 - t3109 * t10860 / 144.0_f64 - t42578 / 36.0_f64;
    t42580
}
