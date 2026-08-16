//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 345/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk345(t1510: f64, t436: f64, t619: f64, t641: f64, t195: f64, t6: f64, t134: f64, t128: f64, t5: f64, t512: f64) -> (f64, f64, f64, f64, f64) {
    let t1511 = t436 * t1510;
    let t1514 = t641 * t619;
    let t1517 = t6 * t195;
    let t1518 = t1517 * t134;
    let t1521 = t1517 * t128;
    let t1524 = t5 * t512;
    (t1511, t1514, t1518, t1521, t1524)
}
