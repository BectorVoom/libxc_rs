//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 697/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk697(t2404: f64, t9578: f64, t92: f64, t2347: f64, t505: f64, t1934: f64) -> (f64, f64, f64, f64) {
    let t9579 = t2404 * t9578;
    let t9580 = t92 * t9579;
    let t9582 = t2347 * t505;
    let t9583 = t9582 * t1934;
    (t9579, t9580, t9582, t9583)
}
