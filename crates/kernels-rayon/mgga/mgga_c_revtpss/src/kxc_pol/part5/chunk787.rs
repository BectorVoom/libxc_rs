//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 787/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk787(t3719: f64, t5230: f64, t247: f64, t1802: f64, t369: f64, t475: f64) -> (f64, f64) {
    let t5385 = t3719 * t5230;
    let t5386 = t247 * t5385;
    let t5389 = t1802 * t369;
    let t5390 = t475 * t5389;
    (t5386, t5390)
}
