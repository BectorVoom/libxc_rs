//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1964/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1964(t20063: f64, t3701: f64, t1484: f64, t2752: f64, t17083: f64, t225: f64, t5584: f64, t852: f64, t1509: f64, t4265: f64, t1519: f64, t4233: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t57806 = t20063 * t3701;
    let t57911 = t2752 * t1484;
    let t58143 = t17083 * t225;
    let t58166 = t852 * t5584;
    let t58204 = t4265 * t1509;
    let t58226 = t1519 * t4233;
    (t57806, t57911, t58143, t58166, t58204, t58226)
}
