//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 523/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk523(t1005: f64, t1103: f64, t1108: f64, t1113: f64, t940: f64, t950: f64, t151: f64, t377: f64, t941: f64, t301: f64, t864: f64, t1089: f64, t175: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3312 = t1005 * t1103;
    let t3314 = t1005 * t1108;
    let t3316 = t1005 * t1113;
    let t3328 = t940 * t950;
    let t3329 = t151 * t3328;
    let t3343 = t377 * t941;
    let t3355 = t864 * t301;
    let t3357 = t1089 * t175 * t3355;
    (t3312, t3314, t3316, t3329, t3343, t3355, t3357)
}
