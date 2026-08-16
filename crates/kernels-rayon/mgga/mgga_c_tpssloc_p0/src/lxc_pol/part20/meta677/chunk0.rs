//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2558/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2558(t14855: f64, t3411: f64, t14933: f64, t300: f64, t1166: f64, t3401: f64, t1155: f64, t3395: f64, t1695: f64, t11292: f64, t1164: f64, t3404: f64, t4857: f64) -> (f64, f64, f64, f64, f64) {
    let t51806 = 0.30762056574649219973e4_f64 * t3411 * t14855;
    let t51807 = t300 * t14933;
    let t51809 = 0.17544670867903938621e1_f64 * t51807 * t1166;
    let t51810 = t300 * t3401;
    let t51811 = t3395 * t1155;
    let t51814 = 0.10526802520742363173e2_f64 * t51810 * t1695 * t51811;
    let t51818 = 0.31168546390226634765e3_f64 * t1164 * t11292 * t4857 * t3404;
    (t51806, t51809, t51811, t51814, t51818)
}
