//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 485/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk485<F: Float>(t1319: F, t1322: F, t1332: F, t1334: F, t1336: F, t1339: F, t1342: F, t1343: F, t1353: F, t1448: F, t1450: F, t198: F, t532: F, t679: F, t704: F) -> F {
    let t1453 = t1448 * t1450 * t198 * t532 + F::new(3.0) * t1343 * t1353 * t198 + t1319 - t1322 + t1332 + t1334 + t1336 - t1339 - t1342 + t679 + t704;
    t1453
}
