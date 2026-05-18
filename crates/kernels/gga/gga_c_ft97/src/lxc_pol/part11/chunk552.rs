//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 552/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk552<F: Float>(t7800: F, t82: F, t7765: F, t1555: F, t89: F, t1557: F, t363: F, t1580: F) -> (F, F, F, F, F) {
    let t7801 = t82 * t7800;
    let t7802 = t7801 * t7765;
    let t7804 = t89 * t1555 * t7802;
    let t7806 = t1557 * t363;
    let t7807 = t7806 * t1580;
    (t7801, t7802, t7804, t7806, t7807)
}
