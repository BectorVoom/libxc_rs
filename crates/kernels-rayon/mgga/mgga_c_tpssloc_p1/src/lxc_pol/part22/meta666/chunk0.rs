//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2221/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2221(t17297: f64, t2932: f64, t2860: f64, t5737: f64, t2841: f64, t5689: f64, t17471: f64, t923: f64, t17488: f64, t892: f64, t17292: f64, t699: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t59895 = t17297 * t2932;
    let t59920 = t5737 * t2860;
    let t59959 = t5689 * t2841;
    let t59962 = t17471 * t923;
    let t59979 = t17488 * t892;
    let t60163 = t699 * t17292;
    (t59895, t59920, t59959, t59962, t59979, t60163)
}
