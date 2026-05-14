//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 407/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk407<F: Float>(t1597: F, t2331: F, t1557: F, t1601: F, t2179: F, t2215: F, t2234: F, t2238: F, t2306: F, t548: F, t2260: F, t2264: F, t2268: F, t2272: F, t2276: F, t2280: F) -> (F, F, F) {
    let t2332 = t2331 * t1597;
    let t2339 = t2306 * t548 - 0.193e0 * t1557 * t2332 + t1601 + 0.11607361111111111111e-2 * t2179 + 0.17411041666666666666e-2 * t2215 - 0.17411041666666666666e-2 * t2234 + 0.11607361111111111111e-2 * t2238;
    let t2347 = 0.9375e-1 * t2260 - 0.9375e-1 * t2264 + 0.625e-1 * t2268 - 0.101171875e-1 * t2272 + 0.101171875e-1 * t2276 - 0.13489583333333333333e-1 * t2280;
    (t2332, t2339, t2347)
}
