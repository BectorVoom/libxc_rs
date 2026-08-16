//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 949/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk949(t1471: f64, t4092: f64, t1701: f64, t213: f64, t1109: f64, t811: f64, t820: f64, t2725: f64, t6: f64, t285: f64, t2726: f64, t3780: f64) -> (f64, f64, f64, f64, f64) {
    let t14721 = t4092 * t1471;
    let t14722 = t1701 * t213;
    let t14723 = t1109 * t811;
    let t14724 = t14723 * t820;
    let t14725 = t14722 * t14724;
    let t14728 = t2725 * t6;
    let t14729 = t285 * t14728;
    let t14730 = t3780 * t2726;
    (t14721, t14725, t14728, t14729, t14730)
}
