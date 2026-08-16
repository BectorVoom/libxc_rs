//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 314/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk314<F: Float>(t104: F, t1637: F, t89: F, t27: F, t444: F, t443: F) -> (F, F) {
    let t1887 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t89 * t1637 * t104;
    let t1900 = t444 * t27;
    let t1901 = t443 * t1900;
    (t1887, t1901)
}
