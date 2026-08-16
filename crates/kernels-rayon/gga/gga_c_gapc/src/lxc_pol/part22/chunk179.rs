//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 179/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk179(t191: f64, t633: f64, t203: f64, t457: f64, t201: f64, t197: f64, t122: f64, t188: f64) -> (f64, f64, f64, f64, f64) {
    let t634 = t633 * t191;
    let t635 = t203 * t457;
    let t636 = t201 * t635;
    let t637 = t197 * t636;
    let t640 = t122 * t188;
    (t634, t635, t636, t637, t640)
}
