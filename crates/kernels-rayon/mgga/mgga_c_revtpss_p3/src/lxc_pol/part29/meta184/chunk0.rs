//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 861/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk861(t3951: f64, t547: f64, t807: f64, t2700: f64, t535: f64, t1369: f64, t794: f64, t1372: f64, t124: f64, t3889: f64, t800: f64, t2453: f64, t546: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3952 = t547 * t3951;
    let t3953 = t807 * t3952;
    let t3956 = 35.0_f64 / 432.0_f64 * t2700 * t535;
    let t3957 = t794 * t1369;
    let t3958 = t3957 * t1372;
    let t3960 = t124 * t3889;
    let t3961 = t800 * t3960;
    let t3964 = t2453 * t546;
    (t3952, t3953, t3956, t3957, t3958, t3961, t3964)
}
