//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1147/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1147(t31443: f64, t35649: f64, t39854: f64, t2288: f64, t8402: f64, t15386: f64, t1745: f64, t2012: f64, t31346: f64, t5903: f64, t35466: f64, t6339: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39856 = t31443 * t35649 * t39854;
    let t39858 = t2288 * t8402;
    let t39860 = t31443 * t15386 * t39858;
    let t39862 = t2012 * t1745;
    let t39867 = t31346 * t5903;
    let t39869 = t35466 * t6339;
    (t39856, t39858, t39860, t39862, t39867, t39869)
}
