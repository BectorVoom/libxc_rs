//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 956/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk956<F: Float>(t24873: F, t4260: F, t15312: F, t1501: F, t2360: F, t3886: F, t15254: F, t2347: F, t15294: F, t15299: F, t28760: F, t684: F, t7045: F, t10703: F, t28516: F, t6334: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t29198 = t24873 * t4260;
    let t29199 = t15312 * t29198;
    let t29202 = t1501 * t2360;
    let t29203 = t29202 * t3886;
    let t29204 = t15254 * t29203;
    let t29207 = t1501 * t2347;
    let t29208 = t29207 * t3886;
    let t29209 = t15294 * t29208;
    let t29212 = t15299 * t28760;
    let t29215 = t7045 * t684;
    let t29216 = t10703 * t29215;
    let t29219 = t15299 * t28516;
    let t29222 = t6334 * t4260;
    (t29198, t29199, t29202, t29203, t29204, t29207, t29208, t29209, t29212, t29215, t29216, t29219, t29222)
}
