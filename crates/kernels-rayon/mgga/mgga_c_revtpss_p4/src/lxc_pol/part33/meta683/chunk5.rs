//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2245/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2245(t21192: f64, t7624: f64, t104636: f64, t104677: f64, t104756: f64, t104768: f64, t104834: f64, t1797: f64, t20825: f64, t20903: f64, t20982: f64, t20986: f64, t26880: f64, t29010: f64, t5279: f64, t5287: f64, t5299: f64, t7618: f64) -> f64 {
    let t112279 = t7624 * t21192;
    let t112299 = -0.38110238327173099531e-3_f64 * t112279 + 0.85748036236139473944e-3_f64 * t104677 * t1797 + t104756 + 0.85748036236139473944e-3_f64 * t29010 * t5287 - 0.45732285992607719436e-2_f64 * t104834 * t1797 + 0.42874018118069736972e-3_f64 * t7618 * t20903 - 0.47637797908966374413e-3_f64 * t26880 * t20825 - 0.11433071498151929859e-2_f64 * t7624 * t20982 - 0.17149607247227894789e-2_f64 * t7624 * t20986 - 0.30488190661738479624e-2_f64 * t104636 * t5299 - 0.30488190661738479624e-2_f64 * t104636 * t5279 + t104768;
    t112299
}
