//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 980/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk980<F: Float>(t10791: F, t1248: F, t2364: F, t11036: F, t2408: F, t4865: F, t11056: F, t1224: F, t4836: F, t6764: F) -> (F, F, F, F) {
    let t17385 = t1248 * t10791 * t2364;
    let t17387 = t11036 * t2408;
    let t17388 = t17387 * t4865;
    let t17390 = t11056 * t2408;
    let t17391 = t17390 * t4865;
    let t17399 = t1224 * t4836 * t6764;
    (t17385, t17388, t17391, t17399)
}
