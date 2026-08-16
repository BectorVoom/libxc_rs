//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2255/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2255(t104647: f64, t104721: f64, t104853: f64, t104888: f64, t104994: f64, t104999: f64, t105002: f64, t105007: f64, t105014: f64, t20767: f64, t20880: f64, t21037: f64, t21173: f64, t21223: f64, t26880: f64, t29097: f64, t29100: f64, t5402: f64) -> f64 {
    let t112531 = -t104994 - 0.19055119163586549765e-3_f64 * t104999 + t105002 - t105007 - 0.57165357490759649296e-3_f64 * t104888 * t5402 + 0.30488190661738479624e-2_f64 * t104721 * t5402 - 0.57165357490759649296e-3_f64 * t29097 * t21223 + 0.28582678745379824648e-3_f64 * t29100 * t21173 + 0.11433071498151929859e-2_f64 * t104647 * t21037 + t105014 + 0.57165357490759649296e-3_f64 * t26880 * t20880 - 0.11433071498151929859e-2_f64 * t104853 * t20767;
    t112531
}
