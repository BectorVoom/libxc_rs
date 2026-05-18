//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 923/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk923<F: Float>(t1882: F, t8541: F, t8546: F, t1917: F, t8232: F, t8503: F, t8480: F, t8421: F, t3281: F, t449: F, t8406: F, t1878: F) -> (F, F, F, F, F, F, F, F, F) {
    let t39051 = t1882 * t8541;
    let t39053 = t1882 * t8546;
    let t39055 = t8232 * t1917;
    let t39057 = t1882 * t8503;
    let t39066 = t1882 * t8480;
    let t39068 = t1882 * t8421;
    let t39093 = t3281 * t449;
    let t39095 = t1882 * t8406;
    let t39097 = t8232 * t1878;
    (t39051, t39053, t39055, t39057, t39066, t39068, t39093, t39095, t39097)
}
