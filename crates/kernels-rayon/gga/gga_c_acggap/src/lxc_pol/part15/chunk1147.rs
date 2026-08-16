//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1147/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1147(t2001: f64, t6228: f64, t6200: f64, t1967: f64, t9573: f64, t17912: f64, t2288: f64, t31443: f64, t8960: f64, t8906: f64, t35649: f64, t8402: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t39844 = t2001 * t6228;
    let t39846 = t2001 * t6200;
    let t39848 = t1967 * t9573;
    let t39852 = t31443 * t17912 * t2288 * t8960;
    let t39854 = t2288 * t8906;
    let t39856 = t31443 * t35649 * t39854;
    let t39858 = t2288 * t8402;
    (t39844, t39846, t39848, t39852, t39854, t39856, t39858)
}
