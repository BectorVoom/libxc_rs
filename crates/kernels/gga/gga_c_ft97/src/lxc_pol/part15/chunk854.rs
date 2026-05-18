//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 854/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk854<F: Float>(t37292: F, t369: F, t7954: F, t1554: F, t1636: F, t1570: F, t174: F, t27: F, t1556: F) -> (F, F, F, F, F, F) {
    let t37293 = F::new(140.0) / F::new(243.0) * t37292;
    let t37305 = t7954 * t369;
    let t37345 = t1636 * t1554;
    let t37352 = F::new(1.0) / t174 / t1570;
    let t37353 = t27 * t37352;
    let t37354 = t1556 * t1556;
    let t37355 = F::new(1.0) / t37354;
    (t37293, t37305, t37345, t37352, t37353, t37355)
}
