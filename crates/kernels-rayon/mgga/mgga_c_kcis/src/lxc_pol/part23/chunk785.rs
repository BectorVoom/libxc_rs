//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 785/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk785(t4078: f64, t743: f64, t4083: f64, t733: f64, t4086: f64, t4096: f64, t1431: f64, t2466: f64, t1438: f64, t2471: f64, t1452: f64, t2475: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11977 = t743 * t4078;
    let t11985 = t733 * t4083;
    let t11987 = t733 * t4086;
    let t11995 = t733 * t4096;
    let t12003 = t2466 * t1431;
    let t12005 = t2471 * t1438;
    let t12009 = t2475 * t1452;
    (t11977, t11985, t11987, t11995, t12003, t12005, t12009)
}
