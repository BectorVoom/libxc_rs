//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1472/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1472<F: Float>(t2198: F, t5787: F, t5517: F, t1312: F, t13426: F, t18227: F, t2199: F, t2201: F, t2322: F, t27123: F, t27126: F, t28219: F, t4248: F, t4254: F, t5523: F, t651: F, t7732: F, t7889: F, t8307: F, t8321: F, t8325: F, t8327: F, t8393: F, t8411: F) -> (F, F, F) {
    let t31382 = t2198 * t5787;
    let t31390 = t5517 * t2198;
    let t31398 = t1312 * t31382 - t13426 * t2199 - t18227 * t2199 - t2199 * t27123 - t2199 * t27126 + t2201 * t28219 - t2322 * t8393 + t2322 * t8411 - t31390 * t651 - t4248 * t8321 - t4254 * t8393 + t5523 * t8411 - t7732 * t8307 - t7732 * t8321 + t7889 * t8325 + t7889 * t8327;
    (t31382, t31390, t31398)
}
