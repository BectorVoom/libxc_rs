//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 931/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk931(t32123: f64, t615: f64, t315: f64, t7930: f64, t2124: f64, t848: f64, t7911: f64, t862: f64, t7898: f64, t13483: f64, t2130: f64, t614: f64) -> (f64, f64, f64, f64, f64) {
    let t32124 = t615 * t32123;
    let t32130 = t315 * t7930;
    let t32135 = t848 * t2124;
    let t32142 = t862 * t7911;
    let t32143 = t32142 * t7898;
    let t32146 = t614 * t13483 * t2130;
    (t32124, t32130, t32135, t32143, t32146)
}
