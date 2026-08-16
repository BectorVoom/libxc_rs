//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1498/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1498(t108710: f64, t109150: f64, t109153: f64, t109242: f64, t1312: f64, t13426: f64, t18227: f64, t2198: f64, t2201: f64, t22506: f64, t2322: f64, t27123: f64, t27126: f64, t29508: f64, t30138: f64, t31390: f64, t31401: f64, t31456: f64, t31459: f64, t31674: f64, t4248: f64, t4254: f64, t7732: f64, t7889: f64, t8307: f64, t8321: f64, t8327: f64, t8393: f64, t8411: f64, t8413: f64) -> f64 {
    let t118864 = 2.0_f64 * t1312 * t2198 * t22506 + 2.0_f64 * t108710 * t2201 + 4.0_f64 * t109150 * t2201 + 4.0_f64 * t109153 * t2201 + 2.0_f64 * t109242 * t2201 + 4.0_f64 * t13426 * t8413 + 4.0_f64 * t18227 * t8411 + 4.0_f64 * t18227 * t8413 - 2.0_f64 * t2322 * t31674 - 4.0_f64 * t27123 * t8393 - 4.0_f64 * t27126 * t8393 - 2.0_f64 * t29508 * t8307 - 2.0_f64 * t29508 * t8321 + 4.0_f64 * t30138 * t8327 - 4.0_f64 * t31390 * t7732 + 4.0_f64 * t31401 * t7889 + 4.0_f64 * t31456 * t4248 + 4.0_f64 * t31456 * t7889 + 4.0_f64 * t31459 * t4248 - 2.0_f64 * t31674 * t4254;
    t118864
}
