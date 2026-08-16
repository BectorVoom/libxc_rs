//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1119/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1119<F: Float>(t10267: F, t681: F, t89: F, t10270: F, t2345: F, t41448: F, t10257: F, t2336: F, t2671: F, t9733: F, t10402: F, t798: F, t9568: F) -> (F, F, F, F, F, F) {
    let t43453 = t89 * t681 * t10267;
    let t43457 = t89 * t2345 * t10270 * t41448;
    let t43460 = t89 * t2336 * t10257;
    let t43463 = t89 * t9733 * t2671;
    let t43466 = t89 * t2336 * t10402;
    let t43468 = t9568 * t798;
    (t43453, t43457, t43460, t43463, t43466, t43468)
}
