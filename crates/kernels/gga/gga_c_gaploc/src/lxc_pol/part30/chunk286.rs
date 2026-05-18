//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 286/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk286<F: Float>(t412: F, t77: F, t136: F, t22: F, t5: F, t83: F, t453: F) -> (F, F, F) {
    let t1194 = t77 * t412;
    let t1197 = F::new(1.0) / t22 / t136;
    let t1198 = t5 * t1197;
    let t1199 = t83 * t1198;
    let t1201 = F::new(1.0) / t453;
    (t1194, t1199, t1201)
}
