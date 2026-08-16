//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 936/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk936<F: Float>(t3138: F, t3142: F, t8498: F, t3139: F, t763: F, t2002: F, t3141: F, t13: F, t2969: F, t6449: F) -> (F, F, F, F) {
    let t8501 = t3138 * t8498 * t3142 / F::cast_from(72.0_f64);
    let t8502 = t3139 * t763;
    let t8506 = t3141 * t2002;
    let t8511 = t6449 * t13 * t2969;
    (t8501, t8502, t8506, t8511)
}
