//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 782/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk782(t1503: f64, t2041: f64, t1165: f64, t1411: f64, t604: f64, t2068: f64, t495: f64, t7381: f64, t7380: f64, t1983: f64, t513: f64, t2095: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8866 = t2041 * t1503;
    let t8869 = t1165 * t604 * t1411;
    let t8870 = t2068 * t8869;
    let t8875 = t7381 * t495;
    let t8876 = t7380 * t8875;
    let t8878 = t1983 * t513;
    let t8879 = t2095 * t8878;
    (t8866, t8869, t8870, t8875, t8876, t8878, t8879)
}
