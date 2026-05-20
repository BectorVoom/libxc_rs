//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1494/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1494<F: Float>(t31027: F, t31143: F, t116: F, t31157: F, t46089: F, t655: F, t10199: F, t2339: F, t2: F, t665: F, t10416: F, t1310: F, t1312: F, t13440: F, t14310: F, t18227: F, t2198: F, t2322: F, t31161: F, t31164: F, t31169: F, t31382: F, t31401: F, t31403: F, t31451: F, t31452: F, t31456: F, t31459: F, t4248: F, t4254: F, t5517: F, t5523: F, t5787: F, t651: F, t7889: F, t8320: F, t8327: F, t8411: F, t8413: F) -> (F, F, F, F, F, F) {
    let t117228 = t31027 * t31143;
    let t117338 = t116 * t31157;
    let t117461 = t46089 * t655;
    let t117544 = t10199 * t2339;
    let t117545 = t2 * t665;
    let t117845 = -F::new(4.0) * t1310 * t31451 * t651 + F::new(2.0) * t1312 * t14310 * t2198 + F::new(4.0) * t1312 * t5787 * t8320 - F::new(4.0) * t5517 * t651 * t8320 + F::new(2.0) * t10416 * t8413 + F::new(2.0) * t13440 * t8411 + F::new(2.0) * t13440 * t8413 + F::new(4.0) * t18227 * t8327 - F::new(4.0) * t2322 * t31403 - F::new(4.0) * t2322 * t31452 + F::new(4.0) * t2322 * t31456 + F::new(4.0) * t31161 * t7889 + F::new(2.0) * t31164 * t4248 - F::new(2.0) * t31169 * t4248 + F::new(4.0) * t31382 * t5523 + F::new(4.0) * t31401 * t5523 - F::new(4.0) * t31403 * t4254 - F::new(4.0) * t31452 * t4254 + F::new(4.0) * t31456 * t5523 + F::new(4.0) * t31459 * t5523;
    (t117228, t117338, t117461, t117544, t117545, t117845)
}
