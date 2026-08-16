//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 986/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk986(t30717: f64, t1998: f64, t4625: f64, t1434: f64, t7736: f64, t1418: f64, t7614: f64, t1083: f64, t1487: f64, t1980: f64, t355: f64, t7458: f64) -> (f64, f64, f64, f64, f64) {
    let t34743 = 35.0_f64 / 108.0_f64 * t30717;
    let t34745 = t1998 * t4625;
    let t34751 = t7736 * t1434;
    let t34753 = t7614 * t1418;
    let t34767 = t1980 * t7458 * t1083 * t355 * t1487;
    (t34743, t34745, t34751, t34753, t34767)
}
