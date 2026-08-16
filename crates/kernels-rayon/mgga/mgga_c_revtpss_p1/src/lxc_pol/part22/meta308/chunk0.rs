//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1746/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1746(t10175: f64, t3917: f64, t3889: f64, t566: f64, t64: f64, t843: f64, t112: f64, t2289: f64, t666: f64, t2341: f64, t625: f64, t2367: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10176 = t10175 * t3917;
    let t10186 = t566 * t3889;
    let t10199 = t64 * t843;
    let t10201 = 154.0_f64 / 27.0_f64 * t10199 * t112;
    let t10202 = t2289 * t666;
    let t10204 = t625 * t2341;
    let t10206 = t625 * t2367;
    (t10176, t10186, t10199, t10201, t10202, t10204, t10206)
}
