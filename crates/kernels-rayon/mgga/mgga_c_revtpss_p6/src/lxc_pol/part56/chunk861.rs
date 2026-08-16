//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 861/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk861(t114: f64, t25822: f64, t25824: f64, t28034: f64, t28037: f64, t28039: f64) -> f64 {
    let t115 = 1.0_f64 < t114;
    let t28042 = piecewise3(t115, 0.0_f64, t25822 + t25824 / 3.0_f64 + t28034 / 3.0_f64 + t28037 / 4.0_f64 - t28039 / 8.0_f64);
    t28042
}
