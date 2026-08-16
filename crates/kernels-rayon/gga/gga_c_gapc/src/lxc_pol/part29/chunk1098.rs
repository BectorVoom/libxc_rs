//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 1098/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk1098(t11808: f64, t9865: f64, t11875: f64, t1453: f64, t7949: f64, t818: f64, t959: f64, t1736: f64, t640: f64, t7073: f64, t1086: f64, t2211: f64, t9388: f64) -> (f64, f64, f64, f64, f64) {
    let t33590 = t11808 * t9865;
    let t33595 = t11875 * t1453 * t818 * t959 * t7949;
    let t33597 = t640 * t1736;
    let t33598 = t7073 * t33597;
    let t33601 = t33598 * t1086 * t2211 * t9388;
    (t33590, t33595, t33597, t33598, t33601)
}
