//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 932/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk932(t2132: f64, t3037: f64, t32146: f64, t609: f64, t2122: f64, t2138: f64, t879: f64, t2131: f64, t847: f64, t7990: f64, t7994: f64, t2130: f64, t851: f64) -> (f64, f64, f64, f64, f64) {
    let t32150 = 0.10408353825846239354e2_f64 * t32146 * t2132 * t609 * t3037;
    let t32157 = t2138 * t2132 * t2122 * t879;
    let t32161 = t2131 * t2132 * t2122 * t847;
    let t32163 = t7990 * t7994;
    let t32165 = t851 * t2130;
    (t32150, t32157, t32161, t32163, t32165)
}
