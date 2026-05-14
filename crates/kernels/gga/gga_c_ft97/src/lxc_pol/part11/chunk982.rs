//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 982/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk982<F: Float>(t2665: F, t43371: F, t446: F, t41482: F, t835: F, t2857: F, t41464: F, t683: F, t7640: F, t10262: F, t684: F, t2409: F, t2682: F, t10248: F, t2667: F, t8232: F) -> (F, F, F, F, F, F, F, F) {
    let t43373 = t446 * t2665 * t43371;
    let t43376 = t446 * t835 * t41482;
    let t43379 = t446 * t2857 * t41464;
    let t43381 = t683 * t7640;
    let t43382 = t684 * t10262;
    let t43384 = t446 * t43381 * t43382;
    let t43386 = t2409 * t2682;
    let t43388 = t446 * t10248 * t43386;
    let t43390 = t8232 * t2667;
    (t43373, t43376, t43379, t43382, t43384, t43386, t43388, t43390)
}
