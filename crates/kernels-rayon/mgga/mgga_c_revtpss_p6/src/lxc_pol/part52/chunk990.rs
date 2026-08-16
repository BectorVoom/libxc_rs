//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 990/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk990(t114: f64, t28034: f64, t25825: f64, t26148: f64, t28037: f64, t28039: f64) -> f64 {
    let t115 = 1.0_f64 < t114;
    let t28679 = 2.0_f64 / 3.0_f64 * t28034;
    let t28683 = piecewise3(t115, 0.0_f64, t26148 + t25825 + t28679 + t28037 / 2.0_f64 - t28039 / 4.0_f64);
    t28683
}
