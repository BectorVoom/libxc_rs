//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1203/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1203<F: Float>(t28963: F, t6210: F, t1466: F, t28977: F, t681: F, t28971: F, t10409: F, t1253: F, t193: F, t2405: F, t2413: F, t25406: F, t2665: F, t2739: F, t29024: F, t29416: F, t6216: F, t6222: F, t6225: F, t6267: F, t6963: F, t6967: F, t98335: F, t98416: F, t98418: F, t98423: F) -> (F,) {
    let t112512 = 2.0 / 9.0 * t6210 * t28963;
    let t112515 = 2.0 / 9.0 * t1466 * t681 * t28977;
    let t112520 = 2.0 / 9.0 * t1466 * t681 * t28971;
    let t112524 = -2.0 / 3.0 * t6963 * t25406 - t1466 * t193 * t6222 * t1253 * t2739 / 3.0 - t6216 * t2665 * t29024 * t2413 / 18.0 - t6216 * t10409 * t29024 * t2405 / 27.0 - t98335 * t6967 / 18.0 + t29416 * t6267 / 3.0 + t112512 + t112515 - 8.0 / 27.0 * t98416 + 2.0 / 9.0 * t98418 + t112520 - 2.0 / 3.0 * t29416 * t6225 + t98423 / 81.0;
    (t112524,)
}
