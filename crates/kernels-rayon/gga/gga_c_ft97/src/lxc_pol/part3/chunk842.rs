//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 842/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk842(t4458: f64, t569: f64, t616: f64, t12752: f64, t17041: f64, t17045: f64, t17049: f64, t17053: f64, t17057: f64, t17060: f64, t17063: f64, t17068: f64, t17073: f64, t17078: f64, t17083: f64, t17088: f64, t17091: f64, t1901: f64, t446: f64) -> f64 {
    let t17095 = t569 * t616 * t4458;
    let t17098 = 2.0_f64 / 27.0_f64 * t1901 * t17041 + 2.0_f64 / 27.0_f64 * t1901 * t17045 + 4.0_f64 / 9.0_f64 * t1901 * t17049 + 2.0_f64 / 9.0_f64 * t1901 * t17053 - 2.0_f64 / 27.0_f64 * t1901 * t17057 + t17060 / 9.0_f64 - 2.0_f64 / 3.0_f64 * t446 * t17063 - 2.0_f64 * t446 * t17068 - 2.0_f64 / 3.0_f64 * t446 * t17073 + 4.0_f64 / 3.0_f64 * t446 * t17078 - 2.0_f64 / 3.0_f64 * t446 * t17083 - 2.0_f64 * t446 * t17088 - 2.0_f64 / 9.0_f64 * t17091 + 8.0_f64 / 27.0_f64 * t12752 + 2.0_f64 / 9.0_f64 * t446 * t17095;
    t17098
}
