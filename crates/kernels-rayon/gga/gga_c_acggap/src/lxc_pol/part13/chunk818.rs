//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 818/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk818(t495: f64, t7381: f64, t7380: f64, t1983: f64, t513: f64, t2095: f64, t2318: f64, t7440: f64, t1323: f64, t142: f64, t7436: f64, t128: f64, t569: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8875 = t7381 * t495;
    let t8876 = t7380 * t8875;
    let t8878 = t1983 * t513;
    let t8879 = t2095 * t8878;
    let t8882 = t7440 * t2318;
    let t8884 = t142 * t1323;
    let t8885 = t7436 * t8884;
    let t8887 = t569 * t128;
    (t8875, t8876, t8878, t8879, t8882, t8884, t8885, t8887)
}
