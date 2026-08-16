//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 559/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk559(t3140: f64, t342: f64, t1034: f64, t358: f64, t360: f64, t368: f64) -> (f64, f64, f64, f64) {
    let t3141 = t342 * t3140;
    let t3143 = 1.0_f64 / t1034 / t358;
    let t3144 = t3143 * t360;
    let t3145 = t368 * t368;
    (t3141, t3143, t3144, t3145)
}
