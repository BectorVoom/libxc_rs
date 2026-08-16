//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1045/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1045(t16190: f64, t49: f64, t75: f64, t10: f64, t47: f64, t204: f64, t5401: f64, t58: f64, t4928: f64, t500: f64, t1476: f64, t170: f64) -> (f64, f64, f64, f64, f64) {
    let t16193 = 0.11483599538271604938e-1_f64 * t49 * t16190 * t75;
    let t16194 = t47 * t10;
    let t16200 = 1.0_f64 / t58 / t16194 * t47 * t5401 * t204 / 48.0_f64;
    let t16202 = t4928 * t500;
    let t16204 = t1476 * t170;
    (t16193, t16194, t16200, t16202, t16204)
}
