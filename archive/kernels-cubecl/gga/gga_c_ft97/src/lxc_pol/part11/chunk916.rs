//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 916/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk916<F: Float>(t38776: F, t38792: F, t38809: F, t38825: F, t1882: F, t8383: F, t8388: F, t8392: F, t482: F, t7943: F, t89: F, t480: F, t8326: F) -> (F, F, F, F, F) {
    let t38827 = t38776 + t38792 + t38809 + t38825;
    let t38833 = t1882 * t8383;
    let t38842 = t8392 * t8388;
    let t38846 = t89 * t7943 * t482;
    let t38866 = t8326 * t480;
    (t38827, t38833, t38842, t38846, t38866)
}
