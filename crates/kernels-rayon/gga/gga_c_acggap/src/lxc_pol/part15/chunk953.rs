//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 953/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk953(t7433: f64, t8869: f64, t7839: f64, t1411: f64, t1992: f64, t7585: f64, t7842: f64, t31699: f64, t8665: f64, t30409: f64, t30418: f64, t31309: f64, t525: f64) -> (f64, f64, f64, f64, f64) {
    let t33841 = t7433 * t8869;
    let t33843 = t7839 * t8869;
    let t33851 = t7585 * t7842 * t1992 * t1411;
    let t33853 = t31699 * t8665;
    let t33857 = t31309 * t30418 * t30409 * t525;
    (t33841, t33843, t33851, t33853, t33857)
}
