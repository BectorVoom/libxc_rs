//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 522/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk522(t2014: f64, t2394: f64, t2710: f64, t4068: f64, t4069: f64, t4073: f64, t4077: f64, t4080: f64, t4083: f64, t807: f64, t291: f64, t800: f64) -> (f64, f64, f64) {
    let t4088 = -0.11705142615505742e0_f64 * t4068 * t4069 + 0.23410285231011484e0_f64 * t4073 - 0.26564305359272358183e-2_f64 * t2014 * t4077 + 0.319782988780431561e-1_f64 * t2710 * t4080 - 0.532971647967385935e-1_f64 * t807 * t4083 + 0.13977476158628290272e-1_f64 * t2394 * t4080;
    let t4089 = t291 * t4088;
    let t4090 = t800 * t4089;
    (t4088, t4089, t4090)
}
