//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2270/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2270(t4021: f64, t641: f64, t72: f64, t645: f64, t7445: f64, t1863: f64, t22550: f64, t7441: f64, t12619: f64, t71: f64, t1860: f64, t22490: f64, t22493: f64, t22512: f64, t22549: f64, t26009: f64, t26021: f64, t26024: f64, t26025: f64, t31683: f64, t6486: f64, t6490: f64, t6505: f64, t7428: f64, t7442: f64, t7446: f64, t9239: f64) -> f64 {
    let t90232 = t72 * t641 * t4021;
    let t90247 = t7445 * t645;
    let t90248 = t1863 * t90247;
    let t90251 = t7441 * t22550;
    let t90257 = t71 * t12619;
    let t90265 = 5.0_f64 / 3.0_f64 * t6490 * t90232 - t22493 * t7446 / 6.0_f64 - t6486 * t26021 / 3.0_f64 - t6486 * t26025 / 3.0_f64 - t1860 * t22512 * t7445 / 6.0_f64 + 20.0_f64 * t9239 * t31683 * t26009 - 10.0_f64 / 3.0_f64 * t22549 * t90248 - 10.0_f64 / 3.0_f64 * t22549 * t90251 - t1860 * t6505 * t26024 / 3.0_f64 - t1860 * t1863 * t90257 / 6.0_f64 - t7428 * t22490 / 6.0_f64 - t22493 * t7442 / 6.0_f64;
    t90265
}
