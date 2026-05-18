//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 750/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk750<F: Float>(t6087: F, t2239: F, t828: F, t2238: F, t348: F, t338: F) -> (F, F, F, F) {
    let t6127 = F::new(0.28842592592592592592e-1) * t6087;
    let t6137 = t828 * t2239;
    let t6141 = F::new(1.0) / t2238 / t348;
    let t6142 = t338 * t6141;
    (t6127, t6137, t6141, t6142)
}
