//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2192/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2192(t100973: f64, t101021: f64, t101064: f64, t101105: f64, t28182: f64, t7235: f64, t13392: f64, t603: f64, t13396: f64, t13405: f64, t4237: f64, t644: f64, t77: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t101107 = t100973 + t101021 + t101064 + t101105;
    let t101124 = 2.0_f64 * t7235 * t28182;
    let t101129 = t603 * t13392;
    let t101132 = t603 * t13396;
    let t101139 = t603 * t13405;
    let t101156 = t77 * t4237 * t644;
    (t101107, t101124, t101129, t101132, t101139, t101156)
}
