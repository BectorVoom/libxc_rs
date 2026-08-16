//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1882/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1882(t2371: f64, t25812: f64, t25814: f64, t25816: f64, t25818: f64, t25820: f64, t25834: f64, t26800: f64, t26804: f64, t27060: f64, t670: f64, t7586: f64) -> f64 {
    let t27066 = 2.0_f64 * t2371 * t7586 + 4.0_f64 * t27060 * t670 + t25812 + t25814 + t25816 + t25818 + t25820 + t25834 + t26800 + 2.0_f64 * t26804;
    t27066
}
