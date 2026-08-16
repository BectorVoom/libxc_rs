//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 583/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk583(t545: f64, t939: f64, t945: f64, t124: f64, t56: f64, t2029: f64, t142: f64, t174: f64) -> (f64, f64, f64, f64) {
    let t4250 = t939 * t545;
    let t4251 = t4250 * t945;
    let t4254 = t124 * t56;
    let t4255 = t4254 * t2029;
    let t4256 = t142 * t174;
    (t4251, t4254, t4255, t4256)
}
