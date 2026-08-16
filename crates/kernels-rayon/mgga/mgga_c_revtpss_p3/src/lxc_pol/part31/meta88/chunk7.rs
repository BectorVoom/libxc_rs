//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 561/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk561(t1368: f64, t1370: f64, t1378: f64, t1383: f64, t1388: f64, t1407: f64, t1410: f64, t1873: f64, t1885: f64, t1889: f64) -> f64 {
    let t1892 = -t1368 - t1370 * t1873 / 48.0_f64 - t1378 + t1383 - 0.21437009059034868486e-3_f64 * t1388 * t1885 - t1407 - 0.85748036236139473944e-3_f64 * t1410 * t1889;
    t1892
}
