//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 711/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk711<F: Float>(t2885: F, t8392: F, t1934: F, t875: F, t2882: F, t2881: F, t4265: F, t9853: F, t4140: F, t4139: F, t2344: F, t798: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10463 = t8392 * t2885;
    let t10465 = t1934 * t875;
    let t10466 = t2882 * t10465;
    let t10467 = t2881 * t10466;
    let t10470 = t4265 * t9853;
    let t10471 = t2881 * t10470;
    let t10474 = t4140 * t9853;
    let t10475 = t4139 * t10474;
    let t10478 = t2344 * t798;
    (t10463, t10465, t10466, t10467, t10470, t10471, t10474, t10475, t10478)
}
