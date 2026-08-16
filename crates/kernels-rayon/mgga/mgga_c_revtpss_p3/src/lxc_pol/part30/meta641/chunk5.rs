//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2233/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2233(t104695: f64, t13142: f64, t17384: f64, t26867: f64, t17640: f64, t17646: f64, t17690: f64, t17705: f64, t17750: f64, t17781: f64, t26852: f64, t29097: f64, t29100: f64, t5304: f64, t5354: f64, t5402: f64, t97182: f64, t97187: f64, t97232: f64) -> f64 {
    let t104774 = t13142 * t104695;
    let t104793 = 0.3811023832717309953e-3_f64 * t26867 * t17384;
    let t104796 = -0.25724410870841842183e-2_f64 * t104774 * t17750 - 0.85748036236139473944e-3_f64 * t97182 * t5354 - 0.57165357490759649296e-3_f64 * t97187 + 0.95275595817932748826e-3_f64 * t26852 * t5304 + 0.85748036236139473944e-3_f64 * t29097 * t17705 - 0.28582678745379824648e-3_f64 * t26867 * t17640 - 0.57165357490759649296e-3_f64 * t26867 * t17646 - 0.57165357490759649296e-3_f64 * t97232 * t5402 + 0.47637797908966374413e-3_f64 * t26867 * t17690 - t104793 - 0.85748036236139473944e-3_f64 * t29100 * t17781;
    t104796
}
