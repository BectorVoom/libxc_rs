//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 833/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk833(t16971: f64, t574: f64, t605: f64, t11593: f64, t12617: f64, t12620: f64, t12642: f64, t12644: f64, t12670: f64, t12672: f64, t12674: f64, t16943: f64, t16947: f64, t16952: f64, t16957: f64, t16960: f64, t16965: f64, t16969: f64, t1901: f64, t446: f64) -> f64 {
    let t16973 = t574 * t605 * t16971;
    let t16976 = -2.0_f64 / 3.0_f64 * t1901 * t16943 + 8.0_f64 / 9.0_f64 * t11593 * t16947 - 2.0_f64 / 9.0_f64 * t1901 * t16952 - 2.0_f64 / 9.0_f64 * t1901 * t16957 + 2.0_f64 / 9.0_f64 * t1901 * t16960 + t1901 * t16965 / 9.0_f64 - 8.0_f64 / 81.0_f64 * t12617 + t12620 - 2.0_f64 / 9.0_f64 * t16969 - t12642 - t12644 + t12670 + t12672 + t12674 + t446 * t16973 / 3.0_f64;
    t16976
}
