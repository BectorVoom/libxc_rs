//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 835/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk835<F: Float>(t25342: F, t25359: F, t295: F, t312: F, t1882: F, t6388: F, t6386: F, t870: F, t684: F, t2881: F, t2409: F, t6273: F, t2874: F, t1501: F, t2844: F, t10697: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t25360 = t25342 + t25359;
    let t25362 = t295 * t25360 * t312;
    let t25366 = t1882 * t6388;
    let t25368 = t870 * t6386;
    let t25369 = t25368 * t684;
    let t25370 = t2881 * t25369;
    let t25373 = t6273 * t2409;
    let t25374 = t2874 * t25373;
    let t25377 = t1501 * t2844;
    let t25378 = t10697 * t25377;
    (t25360, t25362, t25366, t25368, t25369, t25370, t25373, t25374, t25377, t25378)
}
