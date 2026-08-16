//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2753/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2753(t58021: f64, t46278: f64, t10126: f64, t12895: f64, t12915: f64, t1484: f64, t16662: f64, t1877: f64, t2522: f64, t2523: f64, t39483: f64, t4255: f64, t4314: f64, t46213: f64, t5527: f64, t57996: f64, t58005: f64, t58008: f64, t58009: f64, t58020: f64) -> (f64, f64, f64) {
    let t58022 = 0.5848223622634646207e0_f64 * t58021;
    let t58023 = 0.32530743900905219526e-1_f64 * t46278;
    let t58024 = 6.0_f64 * t10126 * t4314 * t5527 + 24.0_f64 * t12895 * t4255 * t4314 + 8.0_f64 * t12915 * t1877 * t58009 + 6.0_f64 * t1484 * t2522 * t46213 + 6.0_f64 * t16662 * t2522 * t2523 + t39483 + t57996 + t58005 + t58008 + t58020 - t58022 + t58023;
    (t58022, t58023, t58024)
}
