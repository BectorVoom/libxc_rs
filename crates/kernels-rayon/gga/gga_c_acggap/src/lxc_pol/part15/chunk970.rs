//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 970/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk970(t1569: f64, t7614: f64, t1988: f64, t8838: f64, t1459: f64, t1980: f64, t33883: f64, t7458: f64, t1967: f64, t8541: f64, t31038: f64, t527: f64) -> (f64, f64, f64, f64, f64) {
    let t34295 = t7614 * t1569;
    let t34297 = t1988 * t8838;
    let t34305 = t1980 * t7458 * t1459 * t33883;
    let t34307 = t1967 * t8541;
    let t34309 = t31038 * t527;
    (t34295, t34297, t34305, t34307, t34309)
}
