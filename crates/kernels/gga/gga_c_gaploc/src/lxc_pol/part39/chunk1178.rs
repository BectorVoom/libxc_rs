//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1178/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1178<F: Float>(t40013: F, t40015: F, t40019: F, t12000: F, t123: F, t883: F, t2487: F, t2488: F, t11981: F, t2464: F, t2465: F, t13782: F, t7014: F) -> (F, F, F, F, F, F, F) {
    let t47873 = F::new(0.63904876589867916128e-1) * t40013;
    let t47874 = F::new(0.63904876589867916128e-1) * t40015;
    let t47875 = F::new(0.63904876589867916128e-1) * t40019;
    let t47877 = t12000 * t123 * t883;
    let t47879 = t2487 * t2488 * t47877;
    let t47883 = t2487 * t2464 * t2465 * t11981;
    let t47885 = t7014 * t13782;
    (t47873, t47874, t47875, t47877, t47879, t47883, t47885)
}
