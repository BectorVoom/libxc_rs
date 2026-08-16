//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1141/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1141(t32063: f64, t34818: f64, t7366: f64, t148403: f64, t39749: f64, t446: f64, t148408: f64, t9073: f64, t148336: f64, t1969: f64, t139497: f64, t3188: f64) -> (f64, f64, f64, f64, f64) {
    let t148464 = t7366 * t32063 * t34818;
    let t148467 = t446 * t39749 * t148403;
    let t148470 = t446 * t9073 * t148408;
    let t148473 = t446 * t1969 * t148336;
    let t148475 = t139497 * t3188;
    (t148464, t148467, t148470, t148473, t148475)
}
