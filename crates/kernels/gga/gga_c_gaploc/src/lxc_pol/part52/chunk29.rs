//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 29/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk29<F: Float>(t110: F, t22: F, t70: F, t75: F, t109: F) -> (F, F, F, F) {
    let t111 = t22 * t110;
    let t112 = F::new(1.0) / t70;
    let t116 = t75 * t75;
    let t118 = F::new(0.19711288999999999999e-2) * t109 * t111 * t112 - F::new(2.0) * t116;
    let t119 = F::new(1.0) / t118;
    (t111, t112, t118, t119)
}
