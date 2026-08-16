//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1164/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1164(t3262: f64, t3276: f64, t42919: f64, t2995: f64, t3250: f64, t3424: f64, t3461: f64, t42871: f64, t42874: f64, t42876: f64, t42881: f64, t42885: f64, t42889: f64, t42893: f64, t42897: f64, t42900: f64, t42904: f64, t42908: f64, t42911: f64, t42914: f64, t42918: f64) -> (f64, f64) {
    let t42922 = 15.0_f64 / 8.0_f64 * t3262 * t3276 * t42919;
    let t42924 = t2995 * t3461 + t3250 * t3424 + t42871 - t42874 + t42876 - t42881 + t42885 + t42889 - t42893 + t42897 - t42900 - t42904 + t42908 + t42911 - t42914 + t42918 - t42922;
    (t42922, t42924)
}
