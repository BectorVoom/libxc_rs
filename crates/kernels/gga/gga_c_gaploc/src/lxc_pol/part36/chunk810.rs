//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 810/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk810<F: Float>(t12656: F, t825: F, t826: F, t12651: F, t12708: F, t7416: F, t2464: F, t2465: F, t2684: F, t9603: F, t2365: F, t28302: F, t7390: F) -> (F, F, F, F, F) {
    let t41425 = t825 * t826 * t12656;
    let t41428 = t825 * t826 * t12651;
    let t41430 = t7416 * t12708;
    let t41435 = t2684 * t2464 * t2465 * t9603;
    let t41445 = t7390 * t2365 * t28302;
    (t41425, t41428, t41430, t41435, t41445)
}
