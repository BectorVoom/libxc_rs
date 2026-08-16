//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 213/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk213(t276: f64, t282: f64, t1: f64, t791: f64, t315: f64, t468: f64, t122: f64) -> (f64, f64, f64, f64, f64) {
    let t792 = t276 * t282;
    let t793 = t792 * t1;
    let t794 = t791 * t793;
    let t795 = t468 * t315;
    let t798 = t792 * t122;
    (t792, t793, t794, t795, t798)
}
