//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 874/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk874(t11981: f64, t1291: f64, t9874: f64, t25: f64, t514: f64, t28: f64, t517: f64, t1376: f64, t68: f64, t522: f64, t9212: f64, t9214: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11982 = 96.0_f64 * t11981;
    let t11984 = 0.56968947174242584612e-3_f64 * t1291 * t9874;
    let t11985 = t25 * t25;
    let t11987 = 1.0_f64 / t514 / t11985;
    let t11998 = t28 * t28;
    let t12000 = 1.0_f64 / t517 / t11998;
    let t12019 = t1376 * t1376;
    let t12020 = 1.0_f64 / t12019;
    let t12021 = t68 * t12020;
    let t12044 = 24.0_f64 * t9212 * t522;
    let t12045 = t9214 * t522;
    (t11982, t11984, t11987, t12000, t12019, t12020, t12021, t12044, t12045)
}
