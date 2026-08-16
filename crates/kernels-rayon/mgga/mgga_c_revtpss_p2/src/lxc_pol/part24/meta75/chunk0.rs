//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 462/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk462(t2231: f64, t27: f64, t592: f64, t596: f64, t21: f64) -> (f64, f64, f64) {
    let t2233 = 30.0_f64 * t2231 * t27;
    let t2235 = 72.0_f64 * t592 * t596;
    let t2236 = t21 * t21;
    (t2233, t2235, t2236)
}
