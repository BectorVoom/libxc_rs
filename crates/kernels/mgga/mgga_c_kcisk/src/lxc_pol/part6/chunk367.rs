//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 367/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk367<F: Float>(t1224: F, t1697: F, t2364: F, t1696: F, t1695: F) -> (F, F, F) {
    let t2402 = t1224 * t1697 * t2364;
    let t2404 = -t1696 - 0.17808333333333333333e-1 * t2402;
    let t2408 = -t1695 / 3.0 - t2402 / 3.0;
    (t2402, t2404, t2408)
}
