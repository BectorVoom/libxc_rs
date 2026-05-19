//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 44/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk44<F: Float>(t43: F, t50: F, t45: F, t47: F, t52: F, zeta_threshold: F) -> (F, F, F, F) {
    let t44 = t43 <= zeta_threshold;
    let t51 = t50 <= zeta_threshold;
    let t96 = t45 * t45;
    let t97 = t47 * t47;
    let t98 = piecewise3::<F>(t44, t96, t97);
    let t99 = t52 * t52;
    let t100 = piecewise3::<F>(t51, t96, t99);
    let t102 = t98 / F::new(2.0) + t100 / F::new(2.0);
    (t96, t97, t99, t102)
}
