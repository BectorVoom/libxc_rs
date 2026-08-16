//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 695/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk695(t114: f64, t6996: f64, t6999: f64) -> (f64, f64) {
    let t115 = 1.0_f64 < t114;
    let t7370 = 2.0_f64 / 3.0_f64 * t6996;
    let t7373 = piecewise3(t115, 0.0_f64, -t7370 - t6999 / 4.0_f64);
    (t7370, t7373)
}
