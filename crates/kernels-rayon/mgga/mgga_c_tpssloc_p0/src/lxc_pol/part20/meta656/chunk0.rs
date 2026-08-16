//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2424/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2424(t49222: f64, t942: f64, t951: f64, t959: f64, t10524: f64, t1580: f64, t42110: f64, t42113: f64, t10723: f64, t13658: f64, t10526: f64, t10623: f64, t13659: f64, t13732: f64, t2940: f64, t4483: f64, t4489: f64, t49278: f64, t49280: f64, t49282: f64, t49426: f64, t49485: f64, t49488: f64, t49491: f64) -> (f64, f64, f64, f64) {
    let t49567 = 0.5848223622634646207e0_f64 * t959 * t942 * t49222 * t951;
    let t49572 = 0.91082604192152556044e5_f64 * t959 * t42110 * t1580 * t42113 * t10524;
    let t49575 = 0.51947577317044391277e2_f64 * t959 * t13658 * t10723;
    let t49585 = t49278 + t49280 + t49282 + 0.35089341735807877242e1_f64 * t2940 * t13732 + t49426 + 0.10389515463408878255e3_f64 * t4483 * t10526 + 0.35089341735807877242e1_f64 * t10623 * t4489 - 0.10389515463408878255e3_f64 * t2940 * t13659 + t49485 - t49488 - t49491;
    (t49567, t49572, t49575, t49585)
}
