//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1486/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1486(t31451: f64, t508: f64, t1911: f64, t8320: f64, t569: f64, t1312: f64, t13426: f64, t18227: f64, t2201: f64, t2322: f64, t27123: f64, t31401: f64, t31403: f64, t31407: f64, t4248: f64, t4254: f64, t5523: f64, t651: f64, t8307: f64, t8325: f64, t8327: f64, t8407: f64, t8413: f64) -> (f64, f64, f64, f64) {
    let t31452 = t508 * t31451;
    let t31456 = t8320 * t1911;
    let t31459 = t31451 * t569;
    let t31461 = t1312 * t31401 + t1312 * t31456 + t1312 * t31459 + t13426 * t2201 + t18227 * t2201 + t2201 * t27123 - t2322 * t8407 + t2322 * t8413 - t31403 * t651 - t31407 * t651 - t31452 * t651 - t4248 * t8307 + t4248 * t8325 + t4248 * t8327 - t4254 * t8407 + t5523 * t8413;
    (t31452, t31456, t31459, t31461)
}
