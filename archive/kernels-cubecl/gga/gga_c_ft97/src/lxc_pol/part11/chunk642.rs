//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 642/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk642<F: Float>(t120: F, t7899: F, t378: F, t8030: F, t72: F, t341: F, t630: F, t343: F, t70: F) -> (F, F, F, F) {
    let t8949 = t7899 * t120;
    let t8950 = t378 * t8949;
    let t8955 = t8030 * t120;
    let t8956 = t72 * t8955;
    let t8959 = t341 * t630;
    let t8963 = t341 * t343 * t70;
    (t8950, t8956, t8959, t8963)
}
