//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 401/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk401(t1571: f64, t1589: f64, t465: f64, t471: f64, t204: f64, t492: f64) -> (f64, f64, f64) {
    let t1590 = t1571 * t1589;
    let t1593 = t465 * t471;
    let t1596 = 0.35616666666666666666e-1_f64 * t204 * t1593 * t492;
    (t1590, t1593, t1596)
}
