//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1117/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1117<F: Float>(t10427: F, t1882: F, t824: F, t9596: F, t2665: F, t446: F, t41490: F, t835: F, t10414: F, t41448: F, t666: F, t89: F) -> (F, F, F, F, F) {
    let t43426 = t1882 * t10427;
    let t43428 = t9596 * t824;
    let t43430 = t446 * t2665 * t43428;
    let t43433 = t446 * t835 * t41490;
    let t43437 = t89 * t666 * t10414 * t41448;
    (t43426, t43428, t43430, t43433, t43437)
}
