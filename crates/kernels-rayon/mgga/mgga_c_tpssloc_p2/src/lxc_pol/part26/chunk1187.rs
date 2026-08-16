//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1187/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1187(t25083: f64, t812: f64, t23077: f64, t6604: f64, t1878: f64, t23033: f64, t253: f64, t254: f64, t10109: f64, t1911: f64, t234: f64, t193: f64, t1962: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t25084 = t812 * t25083;
    let t25119 = t23077 * t6604;
    let t25154 = t1878 * t23033;
    let t25168 = t253 * t254;
    let t25169 = t10109 * t1911;
    let t25248 = t6604 * t234;
    let t25372 = t193 * t1962;
    (t25084, t25119, t25154, t25168, t25169, t25248, t25372)
}
