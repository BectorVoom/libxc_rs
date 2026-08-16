//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2246/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2246(t1936: f64, t85360: f64, t18245: f64, t7002: f64, t109150: f64, t109153: f64, t105866: f64, t108120: f64, t109204: f64, t109222: f64, t1518: f64, t21881: f64, t25805: f64, t28025: f64, t28030: f64, t33602: f64, t4292: f64, t5920: f64, t670: f64, t6985: f64, t97622: f64) -> f64 {
    let t109224 = 2.0_f64 * t85360 * t1936;
    let t109226 = 2.0_f64 * t18245 * t7002;
    let t109228 = 4.0_f64 * t109150 * t1936;
    let t109230 = 4.0_f64 * t109153 * t1936;
    let t109231 = 2.0_f64 * t105866 * t670 + 4.0_f64 * t108120 * t1518 + 4.0_f64 * t1518 * t97622 + 2.0_f64 * t21881 * t6985 + 2.0_f64 * t25805 * t5920 + 2.0_f64 * t28025 * t5920 + 4.0_f64 * t28030 * t4292 + 4.0_f64 * t33602 * t4292 + t109204 + t109222 + t109224 + t109226 + t109228 + t109230;
    t109231
}
