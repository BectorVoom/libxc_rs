//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1246/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1246<F: Float>(t3065: F, t49508: F, t858: F, t8978: F, t12069: F, t13414: F, t3123: F, t46451: F, t11787: F, t36659: F, t36641: F, t13252: F, t37632: F) -> (F, F, F, F, F, F) {
    let t49745 = t8978 * t3065 * t858 * t49508 / F::new(24.0);
    let t49761 = t13414 * t12069 / F::new(4.0);
    let t49763 = t3123 * t46451 / F::new(24.0);
    let t49765 = t36659 * t11787 / F::new(8.0);
    let t49767 = t36641 * t11787 / F::new(8.0);
    let t49768 = t37632 * t13252;
    (t49745, t49761, t49763, t49765, t49767, t49768)
}
