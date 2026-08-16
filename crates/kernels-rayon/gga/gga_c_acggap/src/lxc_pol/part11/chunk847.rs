//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 847/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk847(t5752: f64, t943: f64, t1454: f64, t372: f64, t1182: f64, t1410: f64, t1487: f64, t407: f64, t1539: f64, t1439: f64, t360: f64, t1416: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t22710 = t5752 * t943;
    let t22778 = t1454 * t372;
    let t23045 = t1182 * t1410;
    let t23445 = t407 * t1487;
    let t23688 = t1539 * t1410;
    let t23718 = t1439 * t360;
    let t23736 = t1416 * t372;
    (t22710, t22778, t23045, t23445, t23688, t23718, t23736)
}
