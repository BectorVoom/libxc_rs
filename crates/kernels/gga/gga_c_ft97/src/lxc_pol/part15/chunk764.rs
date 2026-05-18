//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 764/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk764<F: Float>(t21222: F, t224: F, t695: F, t5006: F, t6762: F, t1128: F, t4986: F, t206: F, t5011: F, t214: F, t52: F, t204: F, t21210: F, t41: F) -> (F, F, F, F, F, F) {
    let t21224 = t224 * t695 * t21222;
    let t21225 = t6762 * t5006;
    let t21227 = t4986 * t1128;
    let t21233 = F::new(1.0) / t206 / t5011;
    let t21235 = t52 * t214 * t21233;
    let t21237 = -F::new(0.205601884870781893e1) * t41 * t204 * t21210 - F::new(0.13764695059989835716e0) * t21235;
    (t21224, t21225, t21227, t21233, t21235, t21237)
}
