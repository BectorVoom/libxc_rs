//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1500/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1500(t109150: f64, t109153: f64, t1312: f64, t13426: f64, t18227: f64, t18245: f64, t1911: f64, t2199: f64, t2322: f64, t27123: f64, t30138: f64, t31382: f64, t31390: f64, t31401: f64, t31451: f64, t31452: f64, t31657: f64, t31663: f64, t4248: f64, t5523: f64, t5787: f64, t6934: f64, t7732: f64, t7889: f64, t8307: f64, t8320: f64, t8321: f64, t8325: f64, t8393: f64, t8406: f64, t8413: f64) -> f64 {
    let t118955 = 4.0_f64 * t1312 * t1911 * t31451 + 4.0_f64 * t1312 * t5787 * t8406 + 2.0_f64 * t1312 * t6934 * t8320 - 4.0_f64 * t109150 * t2199 - 4.0_f64 * t109153 * t2199 - 4.0_f64 * t13426 * t8393 - 4.0_f64 * t18227 * t8393 - 2.0_f64 * t18245 * t8321 + 2.0_f64 * t18245 * t8325 + 4.0_f64 * t2322 * t31657 + 2.0_f64 * t2322 * t31663 + 4.0_f64 * t27123 * t8413 - 4.0_f64 * t30138 * t8307 - 4.0_f64 * t30138 * t8321 + 4.0_f64 * t30138 * t8325 + 4.0_f64 * t31382 * t7889 - 4.0_f64 * t31390 * t4248 + 4.0_f64 * t31401 * t4248 - 4.0_f64 * t31452 * t7732 + 4.0_f64 * t31657 * t5523;
    t118955
}
