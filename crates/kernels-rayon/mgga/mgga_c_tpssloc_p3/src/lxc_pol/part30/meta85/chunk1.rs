//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 550/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk550(t1729: f64, t68: f64, t484: f64, t1659: f64, t1673: f64, t1699: f64, t1701: f64, t1705: f64) -> (f64, f64, f64) {
    let t1730 = t1729 * t68;
    let t1731 = t1730 * t484;
    let t1734 = -t1659 + t1673 + t1699 + t1701 - t1705;
    (t1730, t1731, t1734)
}
