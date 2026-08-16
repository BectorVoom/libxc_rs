//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 955/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk955(t2294: f64, t7780: f64, t1524: f64, t355: f64, t1083: f64, t1980: f64, t7458: f64, t535: f64, t7457: f64, t7459: f64, t3201: f64, t8489: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t33876 = t7780 * t2294;
    let t33883 = t355 * t1524;
    let t33884 = t1083 * t33883;
    let t33886 = t1980 * t7458 * t33884;
    let t33894 = t7457 * t7458 * t535 * t7459;
    let t33901 = t3201 * t8489;
    (t33876, t33883, t33884, t33886, t33894, t33901)
}
