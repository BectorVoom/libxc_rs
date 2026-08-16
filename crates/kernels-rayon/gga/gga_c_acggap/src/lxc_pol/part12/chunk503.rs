//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 503/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk503(t2792: f64, t75: f64, t689: f64, t83: f64, t2775: f64, t199: f64, t775: f64, t13: f64, t30: f64, t778: f64, t2666: f64, t27: f64) -> (f64, f64, f64, f64, f64) {
    let t2793 = t75 * t2792;
    let t2795 = 1.0_f64 / t689 / t83;
    let t2796 = t2775 * t2795;
    let t2800 = 1.0_f64 / t775 / t199;
    let t2801 = t13 * t2800;
    let t2803 = 1.0_f64 / t778 / t30;
    let t2804 = t2666 * t2803;
    let t2805 = t2801 * t2804;
    let t2806 = 0.51726012919273400301e3_f64 * t2805;
    let t2808 = 1.0_f64 / t775 / t27;
    (t2793, t2795, t2796, t2806, t2808)
}
