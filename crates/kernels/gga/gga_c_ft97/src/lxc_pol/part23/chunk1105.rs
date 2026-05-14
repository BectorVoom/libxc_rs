//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1105/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1105<F: Float>(t25111: F, t2691: F, t1472: F, t96737: F, t6256: F, t96536: F, t24964: F, t683: F, t2842: F, t6347: F, t6371: F, t8232: F, t6388: F, t6367: F, t6376: F, t10478: F, t1495: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t98563 = t2691 * t25111;
    let t98581 = 0.18521666970164609055e-1 * t1472 * t96737;
    let t98612 = t6256 * t96536;
    let t98694 = t683 * t24964;
    let t98724 = t6347 * t2842;
    let t98751 = t8232 * t6371;
    let t98753 = t8232 * t6388;
    let t98788 = t8232 * t6367;
    let t98790 = t8232 * t6376;
    let t98809 = t10478 * t1495;
    (t98563, t98581, t98612, t98694, t98724, t98751, t98753, t98788, t98790, t98809)
}
