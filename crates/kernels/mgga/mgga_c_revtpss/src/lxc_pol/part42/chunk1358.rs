//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1358/1363 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1358<F: Float>(t109150: F, t109153: F, t1312: F, t13426: F, t18227: F, t18245: F, t1911: F, t2199: F, t2322: F, t27123: F, t30138: F, t31382: F, t31390: F, t31401: F, t31451: F, t31452: F, t31657: F, t31663: F, t4248: F, t5523: F, t5787: F, t6934: F, t7732: F, t7889: F, t8307: F, t8320: F, t8321: F, t8325: F, t8393: F, t8406: F, t8413: F) -> (F,) {
    let t118955 = 4.0 * t1312 * t1911 * t31451 + 4.0 * t1312 * t5787 * t8406 + 2.0 * t1312 * t6934 * t8320 - 4.0 * t109150 * t2199 - 4.0 * t109153 * t2199 - 4.0 * t13426 * t8393 - 4.0 * t18227 * t8393 - 2.0 * t18245 * t8321 + 2.0 * t18245 * t8325 + 4.0 * t2322 * t31657 + 2.0 * t2322 * t31663 + 4.0 * t27123 * t8413 - 4.0 * t30138 * t8307 - 4.0 * t30138 * t8321 + 4.0 * t30138 * t8325 + 4.0 * t31382 * t7889 - 4.0 * t31390 * t4248 + 4.0 * t31401 * t4248 - 4.0 * t31452 * t7732 + 4.0 * t31657 * t5523;
    (t118955,)
}
