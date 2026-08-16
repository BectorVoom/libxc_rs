//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1287/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1287(t14023: f64, t14548: f64, t863: f64, t14547: f64, t28029: f64, t6523: f64, t14031: f64, t9556: f64, t14011: f64, t9344: f64, t850: f64, t852: f64, t9441: f64) -> (f64, f64, f64, f64, f64) {
    let t54329 = t863 * t14023 * t14548;
    let t54333 = t14547 * t6523 * t28029;
    let t54335 = t14031 * t9556;
    let t54338 = t14011 * t9344;
    let t54341 = t850 * t9441 * t852;
    (t54329, t54333, t54335, t54338, t54341)
}
