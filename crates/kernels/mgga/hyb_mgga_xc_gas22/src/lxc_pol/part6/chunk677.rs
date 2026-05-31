//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 677/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk677<F: Float>(t3389: F, t828: F, t2167: F, t2279: F, t3300: F, t3311: F) -> (F, F) {
    let t3390 = t3389 * t828;
    let t3396 = t2279 - F::cast_from(0.92708333333333333333e-2_f64) * t2167 - F::cast_from(0.92708333333333333333e-2_f64) * t3300 + F::cast_from(0.278125e-1_f64) * t3311;
    (t3390, t3396)
}
