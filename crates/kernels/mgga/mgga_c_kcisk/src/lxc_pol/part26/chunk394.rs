//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 394/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk394<F: Float>(t2262: F, t487: F, t1487: F, t2152: F, t382: F, t486: F, t2211: F, t467: F) -> (F, F, F, F, F, F) {
    let t2263 = t487 * t2262;
    let t2264 = t1487 * t2263;
    let t2266 = t382 * t2152;
    let t2267 = t487 * t2266;
    let t2268 = t486 * t2267;
    let t2270 = t2211 * t467;
    (t2263, t2264, t2266, t2267, t2268, t2270)
}
