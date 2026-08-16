//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1196/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1196(t112943: f64, t6562: f64, t7488: f64, t10110: f64, t112863: f64, t118802: f64, t118810: f64, t118814: f64, t118825: f64, t118828: f64, t13042: f64, t13053: f64, t1492: f64, t1912: f64, t23278: f64, t25184: f64, t259: f64, t2597: f64, t30725: f64, t32804: f64, t4300: f64, t6627: f64, t7517: f64, t8352: f64, t8353: f64, t8363: f64, t855: f64, t86988: f64) -> f64 {
    let t118830 = t6562 * t112943 * t7488;
    let t118831 = 0.82246703342411321825e-2_f64 * t118830;
    let t118832 = -6.0_f64 * t10110 * t4300 * t8352 * t855 + t1492 * t259 * t30725 + 2.0_f64 * t13042 * t8353 - t13053 * t8363 - 2.0_f64 * t1912 * t86988 + 4.0_f64 * t23278 * t7517 + 4.0_f64 * t25184 * t6627 + 2.0_f64 * t2597 * t32804 + t112863 + t118802 - t118810 + t118814 + t118825 + t118828 + t118831;
    t118832
}
