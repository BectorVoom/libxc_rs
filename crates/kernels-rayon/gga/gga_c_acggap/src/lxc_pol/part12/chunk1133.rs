//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1133/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1133(t1451: f64, t7614: f64, t2304: f64, t7630: f64, t2294: f64, t7610: f64, t1988: f64, t8497: f64, t8502: f64, t7799: f64, t8506: f64, t2290: f64, t7780: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t36125 = t7614 * t1451;
    let t36127 = t7630 * t2304;
    let t36129 = t7610 * t2294;
    let t36131 = t1988 * t8497;
    let t36133 = t1988 * t8502;
    let t36135 = t7799 * t8506;
    let t36137 = t7780 * t2290;
    (t36125, t36127, t36129, t36131, t36133, t36135, t36137)
}
