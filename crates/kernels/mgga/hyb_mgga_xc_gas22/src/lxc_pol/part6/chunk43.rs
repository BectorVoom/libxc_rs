//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 43/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk43<F: Float>(t43: F, t47: F, t51: F, t54: F, t57: F, t60: F, t63: F, t66: F, t69: F, t72: F, t88: F) -> (F,) {
    let t44 = 0.135e1 <= t43;
    let t92 = piecewise3(t44, 1.0 / t47 / 36.0 - t51 / 960.0 + t54 / 26880.0 - t57 / 829440.0 + t60 / 28385280.0 - t63 / 0.107347968e10 + t66 / 0.445906944e11 - t69 / 0.20214448128e13, 1.0 - 8.0 / 3.0 * t72 * t88);
    (t92,)
}
