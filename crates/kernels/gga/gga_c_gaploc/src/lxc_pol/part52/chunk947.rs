//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 947/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk947<F: Float>(t12092: F, t2482: F, t9267: F, t40009: F, t40013: F, t40015: F, t40019: F, t12000: F, t123: F, t883: F, t2487: F, t2488: F) -> (F, F, F, F, F, F, F) {
    let t47869 = t9267 * t12092 * t2482;
    let t47871 = F::new(0.63904876589867916128e-1) * t40009;
    let t47873 = F::new(0.63904876589867916128e-1) * t40013;
    let t47874 = F::new(0.63904876589867916128e-1) * t40015;
    let t47875 = F::new(0.63904876589867916128e-1) * t40019;
    let t47877 = t12000 * t123 * t883;
    let t47879 = t2487 * t2488 * t47877;
    (t47869, t47871, t47873, t47874, t47875, t47877, t47879)
}
