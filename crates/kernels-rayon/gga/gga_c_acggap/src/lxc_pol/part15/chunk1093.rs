//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1093/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1093(t1844: f64, t301: f64, t1181: f64, t599: f64, t7337: f64, t368: f64, t5659: f64, t7380: f64, t1795: f64, t355: f64, t1083: f64, t2095: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t38883 = t1844 * t301;
    let t38886 = t7337 * t1181 * t599 * t38883;
    let t38889 = t368 * t5659;
    let t38890 = t7380 * t38889;
    let t38892 = t355 * t1795;
    let t38893 = t1083 * t38892;
    let t38894 = t2095 * t38893;
    (t38883, t38886, t38889, t38890, t38892, t38893, t38894)
}
