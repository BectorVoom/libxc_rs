//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1199/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1199<F: Float>(t20469: F, t3065: F, t858: F, t6678: F, t2285: F, t6455: F, t2206: F, t6661: F, t19: F, t2298: F, t56: F, t21011: F, t884: F) -> (F, F, F, F) {
    let t21316 = t3065 * t858 * t20469;
    let t21318 = t6678 * t21316 / F::new(16.0);
    let t21319 = t6455 * t2285;
    let t21325 = t2206 * t6661;
    let t21326 = F::new(7.0) / F::new(3.0) * t21325;
    let t21328 = t56 * t2298 * t19;
    let t21332 = F::new(5.0) / F::new(4.0) * t884 * t21328 * t858 * t21011;
    (t21318, t21319, t21326, t21332)
}
