//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 972/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk972(t2124: f64, t848: f64, t7911: f64, t862: f64, t7898: f64, t13483: f64, t2130: f64, t614: f64, t2132: f64, t3037: f64, t609: f64, t2122: f64, t2138: f64, t879: f64) -> (f64, f64, f64, f64) {
    let t32135 = t848 * t2124;
    let t32142 = t862 * t7911;
    let t32143 = t32142 * t7898;
    let t32146 = t614 * t13483 * t2130;
    let t32150 = 0.10408353825846239354e2_f64 * t32146 * t2132 * t609 * t3037;
    let t32157 = t2138 * t2132 * t2122 * t879;
    (t32135, t32143, t32150, t32157)
}
