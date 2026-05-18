//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1213/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1213<F: Float>(t21535: F, t2250: F, t6640: F, t2216: F, t6480: F, t21447: F, t2155: F, t858: F, t867: F, t19646: F, t346: F, t2124: F, t822: F) -> (F, F, F, F) {
    let t21536 = t2250 * t21535;
    let t21537 = t21536 * t6640;
    let t21539 = t6480 * t2216;
    let t21540 = F::new(35.0) / F::new(36.0) * t21539;
    let t21544 = t2155 * t867 * t858 * t21447 / F::new(16.0);
    let t21560 = t19646 * t346;
    let t21563 = t822 * t21560 * t2124 / F::new(32.0);
    (t21537, t21540, t21544, t21563)
}
