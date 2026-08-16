//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2673/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2673(t1065: f64, t372: f64, t6305: f64, t19912: f64, t3241: f64, t1011: f64, t6292: f64, t697: f64, t11922: f64, t19717: f64, t4899: f64, t11675: f64, t19785: f64) -> (f64, f64, f64, f64, f64) {
    let t66187 = t372 * t1065 * t6305;
    let t66215 = t3241 * t19912;
    let t66218 = t1011 * t697 * t6292;
    let t66221 = t4899 * t11922 * t19717;
    let t66261 = t11675 * t19785;
    (t66187, t66215, t66218, t66221, t66261)
}
