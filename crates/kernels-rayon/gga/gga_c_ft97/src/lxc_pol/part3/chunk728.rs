//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 728/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk728(t13730: f64, t4044: f64, t89: f64, t1471: f64, t4092: f64, t1701: f64, t213: f64, t2725: f64, t6: f64, t285: f64, t1196: f64, t2724: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14718 = t89 * t13730 * t4044;
    let t14721 = t4092 * t1471;
    let t14722 = t1701 * t213;
    let t14728 = t2725 * t6;
    let t14729 = t285 * t14728;
    let t14738 = t2724 * t1196;
    (t14718, t14721, t14722, t14728, t14729, t14738)
}
