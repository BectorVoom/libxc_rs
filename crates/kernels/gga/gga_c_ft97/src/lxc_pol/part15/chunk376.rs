//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 376/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk376<F: Float>(t2: F, t2680: F, t2652: F, t2399: F, t313: F, t89: F, t311: F, t869: F) -> (F, F, F, F, F) {
    let t2781 = t2680 * t2;
    let t2793 = 4.0 / 9.0 * t2652;
    let t2816 = 4.0 / 27.0 * t89 * t2399 * t313;
    let t2823 = 4.0 / 27.0 * t2652;
    let t2842 = 1.0 / t869 / t311;
    (t2781, t2793, t2816, t2823, t2842)
}
