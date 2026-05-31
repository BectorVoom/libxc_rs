//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 291/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk291<F: Float>(t1546: F, t364: F, t89: F, t375: F, t433: F, t174: F, t22: F) -> (F, F, F, F, F) {
    let t1548 = t89 * t1546 * t364;
    let t1549 = t1548 / F::cast_from(27.0_f64);
    let t1551 = t89 * t375 * t433;
    let t1552 = t1551 / F::cast_from(9.0_f64);
    let t1554 = F::cast_from(1.0_f64) / t174 / t22;
    (t1548, t1549, t1551, t1552, t1554)
}
