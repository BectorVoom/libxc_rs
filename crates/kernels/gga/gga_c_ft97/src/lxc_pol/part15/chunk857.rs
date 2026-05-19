//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 857/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk857<F: Float>(t1736: F, t7763: F, t3626: F, t47: F, t68: F, t72: F, t371: F, t8052: F, t19: F, t7: F) -> (F, F, F, F, F) {
    let t37789 = t1736 * t7763;
    let t37818 = t47 * t3626;
    let t37820 = t68 * t37818 * t72;
    let t37821 = F::cast_from(0.18916624705075445817e-1_f64) * t37820;
    let t37835 = t371 * t8052;
    let t37991 = t7 * t19;
    (t37789, t37820, t37821, t37835, t37991)
}
