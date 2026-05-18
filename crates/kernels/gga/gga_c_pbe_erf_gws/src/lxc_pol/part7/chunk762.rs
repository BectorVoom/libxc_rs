//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 762/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk762<F: Float>(t6231: F, t6241: F, t858: F, t867: F, t6240: F, t2157: F, t2155: F, t2306: F, t346: F, t2382: F, t2150: F, t2074: F, t337: F, t5: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t6242 = t6231 * t6241;
    let t6244 = t867 * t858 * t6242;
    let t6246 = t6240 * t6244 / F::new(16.0);
    let t6247 = t6231 * t2157;
    let t6249 = t867 * t858 * t6247;
    let t6251 = t2155 * t6249 / F::new(16.0);
    let t6252 = t2306 * t346;
    let t6253 = t2382 * t6252;
    let t6255 = t6253 * t2150 / F::new(16.0);
    let t6257 = t337 * t5 * t2074;
    (t6242, t6244, t6246, t6247, t6249, t6251, t6252, t6253, t6255, t6257)
}
