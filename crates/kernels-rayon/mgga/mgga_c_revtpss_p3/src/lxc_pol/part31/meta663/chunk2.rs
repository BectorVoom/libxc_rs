//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2247/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2247(t30138: f64, t7002: f64, t13426: f64, t7741: f64, t18227: f64, t28042: f64, t4248: f64, t108710: f64, t1936: f64, t21881: f64, t93: f64, t30143: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t109233 = 4.0_f64 * t30138 * t7002;
    let t109235 = 4.0_f64 * t13426 * t7741;
    let t109237 = 4.0_f64 * t18227 * t7741;
    let t109239 = 4.0_f64 * t4248 * t28042;
    let t109241 = 2.0_f64 * t108710 * t1936;
    let t109242 = t93 * t21881;
    let t109244 = 2.0_f64 * t109242 * t1936;
    let t109246 = 2.0_f64 * t30143 * t7002;
    (t109233, t109235, t109237, t109239, t109241, t109244, t109246)
}
