//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 974/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk974<F: Float>(t152661: F, t24976: F, t24980: F, t1212: F, t143293: F, t193: F, t89: F, t152772: F, t6317: F, t143373: F, t2665: F, t446: F, t992: F, t35854: F, t6308: F, t681: F) -> (F, F, F, F, F) {
    let t152920 = t24980 * t24976 * t152661;
    let t152924 = t89 * t193 * t143293 * t1212;
    let t152927 = t6317 * t24976 * t152772;
    let t152931 = t446 * t2665 * t143373 * t992;
    let t152934 = t6308 * t681 * t35854;
    (t152920, t152924, t152927, t152931, t152934)
}
