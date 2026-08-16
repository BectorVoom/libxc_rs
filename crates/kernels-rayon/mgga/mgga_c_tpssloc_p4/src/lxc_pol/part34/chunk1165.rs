//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1165/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1165(t20882: f64, t23146: f64, t20988: f64, t25084: f64, t20891: f64, t1898: f64, t20937: f64, t249: f64, t20983: f64, t25146: f64, t5619: f64, t5587: f64, t87218: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t105292 = t23146 * t20882;
    let t105294 = t25084 * t20988;
    let t105296 = t23146 * t20891;
    let t105299 = t20937 * t1898 * t249;
    let t105304 = t25084 * t20983;
    let t105309 = t25146 * t5619;
    let t105311 = t87218 * t5587;
    (t105292, t105294, t105296, t105299, t105304, t105309, t105311)
}
