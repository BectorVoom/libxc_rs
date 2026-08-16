//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2047/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2047(t108710: f64, t108714: f64, t109100: f64, t13426: f64, t18227: f64, t18242: f64, t1843: f64, t2014: f64, t2056: f64, t2107: f64, t25082: f64, t26399: f64, t27123: f64, t27126: f64, t28286: f64, t28658: f64, t28683: f64, t28704: f64, t28711: f64, t29508: f64, t30218: f64, t30511: f64, t30586: f64, t4248: f64, t5921: f64, t651: f64, t670: f64, t7235: f64, t73407: f64, t7359: f64, t7367: f64, t7732: f64, t7984: f64) -> f64 {
    let t111174 = -4.0_f64 * t4248 * t28711 - 2.0_f64 * t108710 * t2056 - 2.0_f64 * t108714 * t2056 - 2.0_f64 * t29508 * t7367 - 4.0_f64 * t651 * t1843 * t28683 - 4.0_f64 * t13426 * t7984 - 4.0_f64 * t18227 * t7984 - 4.0_f64 * t4248 * t28704 - 4.0_f64 * t27123 * t7984 - 4.0_f64 * t27126 * t7984 - 4.0_f64 * t7732 * t28704 - 2.0_f64 * t7235 * t30218 + 6.0_f64 * t25082 * t28286 * t109100 - 2.0_f64 * t26399 * t5921 - 2.0_f64 * t28658 * t5921 - 2.0_f64 * t7359 * t18242 - 2.0_f64 * t651 * t30511 * t670 - t2014 * t2107 * t73407 + 6.0_f64 * t7235 * t30586;
    t111174
}
