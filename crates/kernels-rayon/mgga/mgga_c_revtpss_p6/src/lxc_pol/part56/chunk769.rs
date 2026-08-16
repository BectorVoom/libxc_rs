//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 769/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk769(t3: f64, t8970: f64, t573: f64, t8613: f64, t8616: f64, t8773: f64, t197: f64, t532: f64, t1450: f64, t2033: f64, t4146: f64, t565: f64, param_d: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8971 = t3 * t8970;
    let t8975 = param_d * t8970;
    let t8978 = t573 * t8975 + t8613 + t8616 + 6.0_f64 * t8773;
    let t8995 = t197 * t532;
    let t8996 = t2033 * t1450;
    let t9593 = 1.0_f64 / t4146 / t565;
    (t8971, t8975, t8978, t8995, t8996, t9593)
}
