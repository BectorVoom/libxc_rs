//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 429/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk429(t2250: f64, t282: f64, t122: f64, t766: f64, t786: f64) -> (f64, f64) {
    let t2251 = t2250 * t282;
    let t2252 = t2251 * t122;
    let t2253 = t766 * t2252;
    let t2254 = t786 * t786;
    (t2253, t2254)
}
