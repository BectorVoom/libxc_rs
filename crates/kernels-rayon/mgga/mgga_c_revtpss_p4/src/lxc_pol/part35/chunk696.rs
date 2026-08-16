//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 696/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk696(t7076: f64, t8011: f64, t233: f64, t7997: f64, t1957: f64, t1580: f64, t1956: f64, t2067: f64, t213: f64, t257: f64, t7070: f64, t7387: f64, t7390: f64, t7403: f64, t7409: f64, t7411: f64, t7766: f64, t7998: f64, t8007: f64) -> (f64, f64, f64, f64) {
    let t8012 = t7076 * t8011;
    let t8015 = t233 * t7997;
    let t8016 = t1957 * t8015;
    let t8019 = -t7387 + t7390 + 0.65854491829355115987e0_f64 * t213 * t7998 * t257 - 0.65854491829355115987e0_f64 * t7403 * t1580 + t7409 - t7411 - 0.4336814094102599731e0_f64 * t7766 * t2067 + 0.8673628188205199462e0_f64 * t7070 * t8007 + 0.4336814094102599731e0_f64 * t7070 * t8012 - 0.4336814094102599731e0_f64 * t1956 * t8016;
    (t8012, t8015, t8016, t8019)
}
