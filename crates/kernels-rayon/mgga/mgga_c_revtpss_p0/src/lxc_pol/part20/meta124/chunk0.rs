//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 715/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk715(t3390: f64, t3391: f64, t3356: f64, t3358: f64, t3365: f64, t3370: f64, t3374: f64) -> (f64, f64) {
    let t3392 = t3390 * t3391;
    let t3394 = 4.0_f64 / 9.0_f64 * t3356;
    let t3399 = t3394 - 2.0_f64 / 9.0_f64 * t3358 - 2.0_f64 / 9.0_f64 * t3365 + 2.0_f64 / 3.0_f64 * t3370 + t3374 / 3.0_f64;
    (t3392, t3399)
}
