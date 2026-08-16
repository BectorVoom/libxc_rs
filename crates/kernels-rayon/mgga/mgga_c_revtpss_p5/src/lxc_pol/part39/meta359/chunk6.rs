//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1246/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1246(t14711: f64, t14754: f64, t14784: f64, t14811: f64, t14841: f64, t14878: f64, t14889: f64, t14936: f64, t136: f64, t1568: f64, t2457: f64, t2710: f64) -> (f64, f64) {
    let t14939 = t14711 + t14754 + t14784 + t14811 + t14841 + t14878 + t14889 + t14936;
    let t14946 = t1568 * t136;
    let t14948 = t2710 * t14946 * t2457;
    (t14939, t14948)
}
