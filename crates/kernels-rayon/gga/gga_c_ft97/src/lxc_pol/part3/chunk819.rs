//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 819/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk819(t16733: f64, t446: f64, t4668: f64, t7368: f64, t558: f64, t28: f64, t89: f64, t3342: f64, t3408: f64, t1546: f64, t4664: f64, t4660: f64) -> (f64, f64, f64, f64, f64) {
    let t16734 = t446 * t16733;
    let t16736 = t7368 * t4668;
    let t16737 = t16736 * t558;
    let t16739 = t89 * t28 * t16737;
    let t16740 = t3342 * t3408;
    let t16742 = t89 * t28 * t16740;
    let t16745 = t89 * t1546 * t4664;
    let t16748 = t89 * t1546 * t4660;
    (t16734, t16739, t16742, t16745, t16748)
}
