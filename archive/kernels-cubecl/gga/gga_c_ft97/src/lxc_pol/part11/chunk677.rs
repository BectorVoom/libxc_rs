//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 677/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk677<F: Float>(t2142: F, t2190: F, t574: F, t1882: F, t2198: F, t2230: F, t379: F, t569: F, t167: F, t7966: F, t2205: F, t7959: F) -> (F, F, F, F, F) {
    let t9402 = t574 * t2142 * t2190;
    let t9405 = t1882 * t2198;
    let t9408 = t569 * t2230 * t379;
    let t9412 = t569 * t167 * t7966;
    let t9416 = t2205 * t167 * t7959;
    (t9402, t9405, t9408, t9412, t9416)
}
