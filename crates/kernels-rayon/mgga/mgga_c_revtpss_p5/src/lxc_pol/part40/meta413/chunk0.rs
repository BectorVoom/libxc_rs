//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1494/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1494(t31027: f64, t31143: f64, t116: f64, t31157: f64, t46089: f64, t655: f64, t10199: f64, t2339: f64, t2: f64, t665: f64, t10416: f64, t1310: f64, t1312: f64, t13440: f64, t14310: f64, t18227: f64, t2198: f64, t2322: f64, t31161: f64, t31164: f64, t31169: f64, t31382: f64, t31401: f64, t31403: f64, t31451: f64, t31452: f64, t31456: f64, t31459: f64, t4248: f64, t4254: f64, t5517: f64, t5523: f64, t5787: f64, t651: f64, t7889: f64, t8320: f64, t8327: f64, t8411: f64, t8413: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t117228 = t31027 * t31143;
    let t117338 = t116 * t31157;
    let t117461 = t46089 * t655;
    let t117544 = t10199 * t2339;
    let t117545 = t2 * t665;
    let t117845 = -4.0_f64 * t1310 * t31451 * t651 + 2.0_f64 * t1312 * t14310 * t2198 + 4.0_f64 * t1312 * t5787 * t8320 - 4.0_f64 * t5517 * t651 * t8320 + 2.0_f64 * t10416 * t8413 + 2.0_f64 * t13440 * t8411 + 2.0_f64 * t13440 * t8413 + 4.0_f64 * t18227 * t8327 - 4.0_f64 * t2322 * t31403 - 4.0_f64 * t2322 * t31452 + 4.0_f64 * t2322 * t31456 + 4.0_f64 * t31161 * t7889 + 2.0_f64 * t31164 * t4248 - 2.0_f64 * t31169 * t4248 + 4.0_f64 * t31382 * t5523 + 4.0_f64 * t31401 * t5523 - 4.0_f64 * t31403 * t4254 - 4.0_f64 * t31452 * t4254 + 4.0_f64 * t31456 * t5523 + 4.0_f64 * t31459 * t5523;
    (t117228, t117338, t117461, t117544, t117545, t117845)
}
