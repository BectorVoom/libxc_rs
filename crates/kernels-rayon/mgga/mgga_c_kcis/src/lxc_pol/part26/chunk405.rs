//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 405/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk405(t2440: f64, t717: f64, t2459: f64, t97: f64, t684: f64, t127: f64, t129: f64, t130: f64, t2379: f64, t2496: f64, t2500: f64, t2507: f64, t2508: f64, t60: f64, t756: f64, t763: f64, t764: f64, t768: f64) -> (f64, f64, f64) {
    let t2514 = t717 * t2440;
    let t2518 = t97 * t2459;
    let t2522 = t684 * t684;
    let t2526 = -0.43802864444444444443e-3_f64 * t127 * t2496 * t130 - 0.2e-22_f64 * t763 * t2500 * t130 - 0.26281718666666666666e-2_f64 * t127 * t756 * t768 + 0.19711288999999999999e-2_f64 * t2507 * t2508 + 0.19711288999999999999e-2_f64 * t763 * t764 * t768 + 0.39422577999999999998e-2_f64 * t127 * t129 * t2514 - 0.19711288999999999999e-2_f64 * t127 * t129 * t2518 - 4.0_f64 * t2522 - 4.0_f64 * t60 * t2379;
    (t2514, t2518, t2526)
}
