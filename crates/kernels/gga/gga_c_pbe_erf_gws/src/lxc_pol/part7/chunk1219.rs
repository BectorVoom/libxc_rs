//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1219/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1219<F: Float>(t21105: F, t823: F, t850: F, t852: F, t860: F, t21601: F, t2168: F, t8599: F, t16463: F, t333: F, t56: F, t338: F, t348: F) -> (F, F, F, F) {
    let t21632 = t850 * t21105 * t823 * t852 * t860 / F::new(96.0);
    let t21635 = F::new(3.0) / F::new(4.0) * t2168 * t8599 * t21601;
    let t21637 = t16463 * t56 * t333;
    let t21640 = F::new(455.0) / F::new(243.0) * t348 * t21637 * t338;
    (t21632, t21635, t21637, t21640)
}
