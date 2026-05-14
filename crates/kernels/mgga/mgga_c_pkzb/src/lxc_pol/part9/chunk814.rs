//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 814/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk814<F: Float>(t2243: F, t6137: F, t2238: F, t348: F, t338: F) -> (F, F, F) {
    let t6139 = 0.48245938496077605201e2 * t6137 * t2243;
    let t6141 = 1.0 / t2238 / t348;
    let t6142 = t338 * t6141;
    (t6139, t6141, t6142)
}
