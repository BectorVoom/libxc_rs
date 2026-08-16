//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 554/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk554<F: Float>(t7800: F, t82: F, t1586: F, t378: F, t12: F, t52: F, t25: F, t409: F, t29: F, t31: F, t122: F, t170: F, t7239: F) -> (F, F, F, F, F, F) {
    let t7801 = t82 * t7800;
    let t7824 = t378 * t1586;
    let t7853 = t52 * t12;
    let t7876 = t409 * t25;
    let t7905 = F::cast_from(1.0_f64) / t31 / t29;
    let t7906 = t122 * t7905;
    let t7911 = F::cast_from(4.0_f64) * t170 * t7239;
    (t7801, t7824, t7853, t7876, t7906, t7911)
}
