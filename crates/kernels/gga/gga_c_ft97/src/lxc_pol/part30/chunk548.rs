//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 548/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk548<F: Float>(t6140: F, t681: F, t89: F, t1434: F, t6128: F, t1424: F, t2347: F, t1882: F, t6137: F, t6061: F, t668: F, t2360: F) -> (F, F, F, F, F, F, F) {
    let t24499 = t681 * t6140;
    let t24500 = t89 * t24499;
    let t24517 = t1434 * t681 * t6128;
    let t24519 = t1424 * t2347;
    let t24524 = t1882 * t6137;
    let t24526 = t6061 * t668;
    let t24531 = t1424 * t2360;
    (t24499, t24500, t24517, t24519, t24524, t24526, t24531)
}
