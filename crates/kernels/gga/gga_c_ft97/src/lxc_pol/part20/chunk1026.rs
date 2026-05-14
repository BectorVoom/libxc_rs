//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1026/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1026<F: Float>(t25456: F, t25462: F, t25485: F, t6210: F, t6242: F, t6243: F, t96535: F, t54863: F, t6248: F, t24330: F, t25123: F, t24378: F, t25070: F, t25072: F, t25127: F, t6249: F) -> (F, F, F, F, F, F, F) {
    let t98423 = t25462 * t25456;
    let t98429 = t6210 * t25485;
    let t98432 = t6242 * t96535 * t6243;
    let t98434 = t54863 * t6248;
    let t98438 = t6242 * t24330 * t25123;
    let t98446 = t25070 * t24378 * t25072;
    let t98462 = t6249 * t24330 * t25127;
    (t98423, t98429, t98432, t98434, t98438, t98446, t98462)
}
