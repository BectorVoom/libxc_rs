//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2243/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2243(t21233: f64, t7624: f64, t29083: f64, t5378: f64, t21090: f64, t26867: f64, t104703: f64, t104708: f64, t104715: f64, t104774: f64, t17183: f64, t20959: f64, t20963: f64, t21030: f64, t21246: f64, t29040: f64, t29096: f64, t5335: f64, t5348: f64, t5354: f64, t5397: f64, t97141: f64, t97261: f64) -> f64 {
    let t112232 = t7624 * t21233;
    let t112234 = t29083 * t5378;
    let t112243 = t26867 * t21090;
    let t112249 = -0.85748036236139473944e-3_f64 * t104703 * t5348 + 0.25724410870841842183e-2_f64 * t104715 * t20959 - 0.25724410870841842183e-2_f64 * t104774 * t20963 + 0.63517063878621832551e-4_f64 * t97141 + 0.31758531939310916275e-3_f64 * t112232 + 0.20325460441158986416e-2_f64 * t112234 - 0.85748036236139473944e-3_f64 * t17183 * t29096 * t5335 + 0.45732285992607719436e-2_f64 * t104708 * t5354 + 0.85748036236139473944e-3_f64 * t29040 * t21246 - 0.38110238327173099531e-3_f64 * t112243 + 0.30488190661738479624e-2_f64 * t29083 * t5397 + 0.85748036236139473944e-3_f64 * t97261 * t21030;
    t112249
}
