//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1481/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1481(t10199: f64, t655: f64, t2198: f64, t5787: f64, t5517: f64, t1312: f64, t13426: f64, t18227: f64, t2199: f64, t2201: f64, t2322: f64, t27123: f64, t27126: f64, t28219: f64, t4248: f64, t4254: f64, t5523: f64, t651: f64, t7732: f64, t7889: f64, t8307: f64, t8321: f64, t8325: f64, t8327: f64, t8393: f64, t8411: f64) -> (f64, f64, f64, f64) {
    let t31287 = t10199 * t655;
    let t31382 = t2198 * t5787;
    let t31390 = t5517 * t2198;
    let t31398 = t1312 * t31382 - t13426 * t2199 - t18227 * t2199 - t2199 * t27123 - t2199 * t27126 + t2201 * t28219 - t2322 * t8393 + t2322 * t8411 - t31390 * t651 - t4248 * t8321 - t4254 * t8393 + t5523 * t8411 - t7732 * t8307 - t7732 * t8321 + t7889 * t8325 + t7889 * t8327;
    (t31287, t31382, t31390, t31398)
}
