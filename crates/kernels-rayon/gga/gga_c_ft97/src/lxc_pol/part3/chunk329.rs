//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 329/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk329(t1557: f64, t1736: f64, t1570: f64, t422: f64, t95: f64, t96: f64, t1542: f64, t9: f64) -> (f64, f64, f64, f64) {
    let t1737 = t1736 * t1557;
    let t1742 = t422 * t1570;
    let t1766 = 1.0_f64 / t96 / t95;
    let t1771 = t9 * t1542;
    (t1737, t1742, t1766, t1771)
}
