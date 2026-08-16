//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 886/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk886(t1340: f64, t9323: f64, t215: f64, t681: f64, t268: f64, t702: f64) -> (f64, f64, f64) {
    let t9325 = 0.51947577317044391277e2_f64 * t1340 * t9323;
    let t9326 = t215 * t681;
    let t9329 = 0.71233333333333333332e-1_f64 * t268 * t9326 * t702;
    (t9325, t9326, t9329)
}
