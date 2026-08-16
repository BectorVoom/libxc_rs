//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2189/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2189<F: Float>(t3862: F, t6379: F, t5293: F, t53945: F, t19921: F, t3866: F, t19926: F, t12300: F, t6417: F, t19868: F, t3799: F, t12283: F, t19958: F) -> (F, F, F, F, F, F, F) {
    let t57383 = t6379 * t3862;
    let t57392 = t53945 * t5293;
    let t57396 = t3866 * t19921;
    let t57398 = t3866 * t19926;
    let t57407 = t12300 * t6417;
    let t57409 = t3799 * t19868;
    let t57437 = t12283 * t19958;
    (t57383, t57392, t57396, t57398, t57407, t57409, t57437)
}
