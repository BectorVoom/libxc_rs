//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 137/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk137(t360: f64, t5: f64, t129: f64, t145: f64, t126: f64, t19: f64, t124: f64) -> (f64, f64, f64, f64) {
    let t361 = t5 * t360;
    let t363 = t129 * t361 * t145;
    let t366 = t126 * t19;
    let t367 = t124 * t366;
    (t361, t363, t366, t367)
}
