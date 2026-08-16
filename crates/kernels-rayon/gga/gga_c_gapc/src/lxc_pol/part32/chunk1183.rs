//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 1183/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk1183(t11543: f64, t8751: f64, t11425: f64, t3085: f64, t3664: f64, t8903: f64, t3691: f64, t8728: f64, t1023: f64, t1386: f64, t3669: f64, t11578: f64, t1952: f64, t619: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34613 = t11543 * t8751;
    let t34615 = t11425 * t3085;
    let t34617 = t3664 * t8903;
    let t34619 = t3691 * t8728;
    let t34622 = t1386 * t3669 * t1023;
    let t34625 = t11578 * t1952 * t619;
    (t34613, t34615, t34617, t34619, t34622, t34625)
}
