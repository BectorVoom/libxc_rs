//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 43/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk43<F: Float>(t43: F, t47: F, t51: F, t54: F, t57: F, t60: F, t63: F, t66: F, t69: F, t72: F, t88: F) -> F {
    let t44 = F::cast_from(0.135e1_f64) <= t43;
    let t92 = piecewise3::<F>(t44, F::cast_from(1.0_f64) / t47 / F::cast_from(36.0_f64) - t51 / F::cast_from(960.0_f64) + t54 / F::cast_from(26880.0_f64) - t57 / F::cast_from(829440.0_f64) + t60 / F::cast_from(28385280.0_f64) - t63 / F::cast_from(0.107347968e10_f64) + t66 / F::cast_from(0.445906944e11_f64) - t69 / F::cast_from(0.20214448128e13_f64), F::cast_from(1.0_f64) - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t72 * t88);
    t92
}
