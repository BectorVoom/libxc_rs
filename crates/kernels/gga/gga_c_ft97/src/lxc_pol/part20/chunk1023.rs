//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1023/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1023<F: Float>(t1466: F, t1478: F, t9555: F, t2404: F, t6222: F, t24989: F, t683: F, t25434: F, t681: F, t25439: F, t25488: F, t6210: F, t25415: F, t25462: F, t2399: F, t6262: F) -> (F, F, F, F, F, F, F, F) {
    let t98257 = 14.0 / 81.0 * t1466 * t9555 * t1478;
    let t98268 = t2404 * t6222;
    let t98273 = t683 * t24989;
    let t98278 = t1466 * t681 * t25434;
    let t98281 = t1466 * t681 * t25439;
    let t98283 = t6210 * t25488;
    let t98297 = t25462 * t25415;
    let t98306 = t1466 * t2399 * t6262;
    (t98257, t98268, t98273, t98278, t98281, t98283, t98297, t98306)
}
