//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 699/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk699(t7488: f64, t2061: f64, t361: f64, t2060: f64, t3360: f64, t7336: f64) -> (f64, f64, f64, f64) {
    let t7489 = 0.305625e-1_f64 * t7488;
    let t7490 = t361 * t2061;
    let t7491 = t2060 * t7490;
    let t7493 = t3360 * t7336;
    (t7489, t7490, t7491, t7493)
}
