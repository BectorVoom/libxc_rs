//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 876/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk876<F: Float>(t1986: F, t2075: F, t28: F, t7368: F, t89: F, t356: F, t37391: F, t519: F, t143: F, t37406: F, t37357: F, t7761: F, t39767: F, t39772: F, t39776: F, t39781: F, t39784: F, t39788: F, t39792: F, t39796: F, t40265: F, t40270: F, t40273: F, t40283: F) -> (F, F, F, F) {
    let t40288 = t89 * t28 * t7368 * t1986 * t2075;
    let t40292 = t89 * t356 * t519 * t37391;
    let t40294 = t143 * t37406;
    let t40297 = t89 * t7761 * t40294 * t37357;
    let t40299 = 8.0 * t39767 + 6.0 * t39772 - 8.0 * t39776 - 80.0 / 81.0 * t39781 + 8.0 / 3.0 * t39784 + 8.0 * t39788 + 2.0 * t39792 - 2.0 / 3.0 * t39796 - t40265 + 24.0 * t40270 + 112.0 / 27.0 * t40273 - 15.0 / 16.0 * t40283 - 36.0 * t40288 - t40292 / 3.0 + 40.0 / 9.0 * t40297;
    (t40288, t40292, t40297, t40299)
}
