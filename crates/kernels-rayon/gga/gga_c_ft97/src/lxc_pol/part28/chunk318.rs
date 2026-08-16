//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 318/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk318(t2179: f64, t3483: f64, t144: f64, t1047: f64, t376: f64, t89: f64, t1039: f64, t2086: f64, t590: f64, t91: f64, t1033: f64, t1775: f64) -> (f64, f64, f64, f64) {
    let t3484 = t2179 * t3483;
    let t3485 = t144 * t3484;
    let t3489 = t89 * t376 * t1047;
    let t3491 = t2086 * t1039;
    let t3493 = t91 * t3491 * t590;
    let t3497 = t1775 * t1033;
    (t3485, t3489, t3493, t3497)
}
