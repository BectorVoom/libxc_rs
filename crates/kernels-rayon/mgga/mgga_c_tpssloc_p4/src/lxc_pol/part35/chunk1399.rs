//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1399/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1399(t1842: f64, t22635: f64, t26331: f64, t96922: f64, t1992: f64, t26354: f64, t6460: f64, t22633: f64, t97637: f64, t1375: f64, t20029: f64, t2015: f64, t2016: f64, t20613: f64, t20661: f64, t28220: f64, t3887: f64, t5215: f64, t6958: f64, t74930: f64, t7729: f64, t90551: f64, t90582: f64, t96920: f64, t97503: f64) -> (f64, f64) {
    let t106991 = t26331 * t22635 * t96922 * t1842;
    let t107007 = t1992 * t22635 * t26354 * t6460;
    let t107015 = t22633 * t22635 * t97637 * t1842;
    let t107024 = -0.23029076935875170111e0_f64 * t96920 + 6.0_f64 * t6958 * t20613 - 0.15626873635058151147e0_f64 * t90551 + 0.49348022005446793095e-1_f64 * t107007 + 12.0_f64 * t5215 * t28220 - t74930 * t2016 + 0.78134368175290755733e-1_f64 * t90582 + 0.49348022005446793095e-1_f64 * t107015 + 12.0_f64 * t20029 * t7729 - 0.49348022005446793095e-1_f64 * t97503 + 2.0_f64 * t1375 * t3887 * t2015 * t20661;
    (t106991, t107024)
}
