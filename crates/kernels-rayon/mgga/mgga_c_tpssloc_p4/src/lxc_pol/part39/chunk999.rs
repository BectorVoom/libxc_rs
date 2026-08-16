//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 999/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk999(t11998: f64, t517: f64, t1376: f64, t68: f64, t225: f64, t3753: f64, t3880: f64, t522: f64, t9212: f64, t9214: f64, t3824: f64, t592: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12000 = 1.0_f64 / t517 / t11998;
    let t12019 = t1376 * t1376;
    let t12020 = 1.0_f64 / t12019;
    let t12021 = t68 * t12020;
    let t12030 = t3753 * t225;
    let t12033 = t3880 * t225;
    let t12044 = 24.0_f64 * t9212 * t522;
    let t12045 = t9214 * t522;
    let t12048 = 12.0_f64 * t592 * t3824;
    (t12000, t12021, t12030, t12033, t12044, t12045, t12048)
}
