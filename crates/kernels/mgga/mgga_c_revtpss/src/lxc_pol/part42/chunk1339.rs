//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1339/1363 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1339<F: Float>(t31451: F, t508: F, t1911: F, t8320: F, t569: F, t1312: F, t13426: F, t18227: F, t2201: F, t2322: F, t27123: F, t31401: F, t31403: F, t31407: F, t4248: F, t4254: F, t5523: F, t651: F, t8307: F, t8325: F, t8327: F, t8407: F, t8413: F) -> (F, F, F, F) {
    let t31452 = t508 * t31451;
    let t31456 = t8320 * t1911;
    let t31459 = t31451 * t569;
    let t31461 = t1312 * t31401 + t1312 * t31456 + t1312 * t31459 + t13426 * t2201 + t18227 * t2201 + t2201 * t27123 - t2322 * t8407 + t2322 * t8413 - t31403 * t651 - t31407 * t651 - t31452 * t651 - t4248 * t8307 + t4248 * t8325 + t4248 * t8327 - t4254 * t8407 + t5523 * t8413;
    (t31452, t31456, t31459, t31461)
}
