//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1086/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1086<F: Float>(t8392: F, t9805: F, t9810: F, t10071: F, t681: F, t89: F, t9976: F, t2571: F, t8232: F, t2471: F, t10067: F, t1882: F) -> (F, F, F, F, F, F, F) {
    let t42648 = t8392 * t9805;
    let t42650 = t8392 * t9810;
    let t42652 = t8392 * t10071;
    let t42690 = t89 * t681 * t9976;
    let t42697 = t8232 * t2571;
    let t42703 = t8232 * t2471;
    let t42708 = t1882 * t10067;
    (t42648, t42650, t42652, t42690, t42697, t42703, t42708)
}
