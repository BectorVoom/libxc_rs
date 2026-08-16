//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2210/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2210(t108710: f64, t1936: f64, t21881: f64, t93: f64, t30143: f64, t7002: f64, t27123: f64, t7741: f64, t28219: f64, t28042: f64, t7889: f64, t2322: f64, t30004: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t109241 = 2.0_f64 * t108710 * t1936;
    let t109242 = t93 * t21881;
    let t109244 = 2.0_f64 * t109242 * t1936;
    let t109246 = 2.0_f64 * t30143 * t7002;
    let t109248 = 4.0_f64 * t27123 * t7741;
    let t109250 = 4.0_f64 * t28219 * t7741;
    let t109252 = 4.0_f64 * t7889 * t28042;
    let t109254 = 2.0_f64 * t2322 * t30004;
    (t109241, t109244, t109246, t109248, t109250, t109252, t109254)
}
