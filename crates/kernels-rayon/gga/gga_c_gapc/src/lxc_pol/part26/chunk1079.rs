//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 1079/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk1079(t11597: f64, t3408: f64, t9563: f64, t1084: f64, t1971: f64, t9841: f64, t9846: f64, t11910: f64, t29350: f64, t125: f64, t8448: f64, t9059: f64) -> (f64, f64, f64, f64) {
    let t33390 = t9563 * t11597 * t3408;
    let t33394 = t1084 * t1971 * t9841 * t9846;
    let t33396 = t11910 * t29350;
    let t33399 = t9059 * t8448 * t125;
    (t33390, t33394, t33396, t33399)
}
