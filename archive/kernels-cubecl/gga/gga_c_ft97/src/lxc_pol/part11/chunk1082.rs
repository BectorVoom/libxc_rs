//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1082/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1082<F: Float>(t2581: F, t8232: F, t10055: F, t1882: F, t10159: F, t192: F, t33300: F, t9819: F, t2528: F, t255: F, t42123: F, t10031: F) -> (F, F, F, F, F, F, F, F) {
    let t42491 = t8232 * t2581;
    let t42493 = t1882 * t10055;
    let t42498 = t1882 * t10159;
    let t42500 = t192 * t33300;
    let t42509 = t1882 * t9819;
    let t42511 = t8232 * t2528;
    let t42517 = t42123 * t255;
    let t42546 = t1882 * t10031;
    (t42491, t42493, t42498, t42500, t42509, t42511, t42517, t42546)
}
