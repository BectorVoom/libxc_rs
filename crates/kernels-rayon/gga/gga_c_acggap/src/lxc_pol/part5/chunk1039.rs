//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1039/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1039(t13064: f64, t500: f64, t171: f64, t3300: f64, t3775: f64, t4360: f64, t1036: f64, t1089: f64, t1298: f64, t175: f64, t864: f64, t1423: f64, t3770: f64) -> (f64, f64, f64, f64, f64) {
    let t17902 = t13064 * t500;
    let t17912 = t171 * t3300;
    let t17921 = t3775 * t4360;
    let t17926 = t1036 * t1089 * t175 * t1298 * t864;
    let t17928 = t3770 * t1423;
    (t17902, t17912, t17921, t17926, t17928)
}
