//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2002/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2002(t13036: f64, t225: f64, t4119: f64, t828: f64, t1484: f64, t2678: f64, t1509: f64, t2631: f64, t9975: f64, t2710: f64, t4233: f64, t852: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t46508 = t13036 * t225;
    let t46565 = t4119 * t828;
    let t46644 = t1484 * t2678;
    let t46693 = t1509 * t2678;
    let t47012 = t1484 * t2631;
    let t47262 = t1509 * t2631;
    let t47285 = t1509 * t9975;
    let t47425 = t2710 * t1509;
    let t47439 = t852 * t4233;
    (t46508, t46565, t46644, t46693, t47012, t47262, t47285, t47425, t47439)
}
