//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1414/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1414(t43756: f64, t43853: f64, t43894: f64, t43909: f64, t1147: f64, t1156: f64, t1164: f64, t11940: f64, t11947: f64, t1254: f64, t193: f64, t336: f64, t3633: f64, t3637: f64, t3640: f64, t43670: f64, t43672: f64, t43674: f64, t43678: f64, t43683: f64, t43685: f64, t43687: f64, t43695: f64, t43702: f64, t43703: f64, t43706: f64, t4700: f64) -> (f64, f64, f64) {
    let t43911 = t43756 + t43853 + t43894 + t43909;
    let t43915 = 0.5848223622634646207e0_f64 * t1164 * t1147 * t43911 * t1156;
    let t43920 = -4.0_f64 * t11940 * t1254 * t3640 * t4700 + 12.0_f64 * t11947 * t3633 * t3637 * t4700 - 6.0_f64 * t193 * t336 * t43703 * t43706 - t43670 - t43672 + t43674 - t43678 - t43683 + t43685 - t43687 - t43695 - t43702 - t43915;
    (t43911, t43915, t43920)
}
