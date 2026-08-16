//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1072/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1072(t1458: f64, t640: f64, t103: f64, t4054: f64, t1: f64, t102: f64, t1509: f64, t681: f64, t1689: f64, t1302: f64, t1457: f64, t126: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14940 = t1458 * t640;
    let t15260 = t4054 * t103;
    let t15284 = t681 * t1 * t102 * t1509;
    let t15341 = t1689 * t1509;
    let t15354 = t1302 * t1457;
    let t15355 = t15354 * t126;
    (t14940, t15260, t15284, t15341, t15354, t15355)
}
