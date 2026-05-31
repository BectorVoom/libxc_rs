//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 30/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk30<F: Float>(t12: F, t43: F, t44: F, t52: F, t41: F, t42: F, t47: F, t38: F, rho0: F, sigma0: F) -> (F, F, F, F) {
    let t53 = sigma0 * sigma0;
    let t54 = t12 * t53;
    let t55 = t43 * t43;
    let t56 = t55 * rho0;
    let t58 = F::cast_from(1.0_f64) / t44 / t56;
    let t60 = t52 * t54 * t58;
    let t61 = F::cast_from(0.55569193573523559258e-3_f64) * t60;
    let t62 = F::cast_from(1.0_f64) + F::cast_from(0.45058854638888888889e-1_f64) * t41 * t42 * t47 + t61;
    let t63 = t62 * t62;
    let t64 = t38 * t63;
    (t60, t61, t63, t64)
}
