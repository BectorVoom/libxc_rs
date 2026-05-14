//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 402/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk402<F: Float>(t2213: F, t471: F, t415: F, t1422: F, t1423: F, t2059: F, t1428: F, t2083: F, t457: F, t1433: F, t2191: F, t1419: F, t1421: F, t2110: F, t338: F, t456: F) -> (F, F, F, F, F, F, F, F) {
    let t2214 = t2213 * t471;
    let t2215 = t415 * t2214;
    let t2218 = t1422 * t1423 * t2059;
    let t2221 = t1428 * t2083;
    let t2222 = t457 * t2221;
    let t2225 = t1433 * t2191;
    let t2226 = t457 * t2225;
    let t2231 = t1419 + 0.65704296666666666667e-3 * t1421 * t2218 + 0.1478346675e-2 * t456 * t2222 - 0.98556445e-3 * t456 * t2226 - 4.0 * t338 * t2110;
    (t2214, t2215, t2218, t2221, t2222, t2225, t2226, t2231)
}
