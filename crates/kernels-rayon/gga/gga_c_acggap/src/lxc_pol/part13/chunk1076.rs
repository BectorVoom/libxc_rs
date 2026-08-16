//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1076/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1076(t1165: f64, t19834: f64, t2068: f64, t604: f64, t1083: f64, t1487: f64, t1980: f64, t355: f64, t7458: f64, t2118: f64, t4999: f64, t7799: f64, t8571: f64) -> (f64, f64, f64, f64) {
    let t34762 = t2068 * t1165 * t604 * t19834;
    let t34767 = t1980 * t7458 * t1083 * t355 * t1487;
    let t34769 = t2118 * t4999;
    let t34771 = t7799 * t8571;
    (t34762, t34767, t34769, t34771)
}
