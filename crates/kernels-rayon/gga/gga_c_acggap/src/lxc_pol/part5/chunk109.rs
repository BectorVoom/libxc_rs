//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 109/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk109(t265: f64, t75: f64, t205: f64, t207: f64, t211: f64, t216: f64) -> (f64, f64) {
    let t266 = t75 * t265;
    let t271 = -0.86308333333333333334e0_f64 * t205 - 0.301925e0_f64 * t207 - 0.5501625e-1_f64 * t211 - 0.82785e-1_f64 * t216;
    (t266, t271)
}
