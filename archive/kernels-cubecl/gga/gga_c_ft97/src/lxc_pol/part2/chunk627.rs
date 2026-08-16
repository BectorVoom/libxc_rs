//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 627/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk627<F: Float>(t1595: F, t1597: F, t25: F, t409: F, t1602: F, t35: F, t401: F, t428: F, t29: F, t31: F, t122: F, t170: F, t7239: F) -> (F, F, F, F, F, F) {
    let t7858 = t1595 * t1597;
    let t7876 = t409 * t25;
    let t7877 = t1602 * t7876;
    let t7878 = t35 * t401;
    let t7879 = t7878 * t428;
    let t7905 = F::cast_from(1.0_f64) / t31 / t29;
    let t7906 = t122 * t7905;
    let t7911 = F::cast_from(4.0_f64) * t170 * t7239;
    (t7858, t7876, t7877, t7879, t7906, t7911)
}
