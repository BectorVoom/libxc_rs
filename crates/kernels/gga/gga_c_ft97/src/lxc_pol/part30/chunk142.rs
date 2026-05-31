//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 142/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk142<F: Float>(t845: F, t856: F, t91: F, t790: F, t795: F, t827: F) -> (F, F, F) {
    let t858 = t91 * t845 * t856;
    let t860 = t790 / F::cast_from(9.0_f64);
    let t863 = t858 / F::cast_from(6.0_f64) - t860 - t795 / F::cast_from(9.0_f64) - t827 / F::cast_from(3.0_f64);
    (t858, t860, t863)
}
