//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1416/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1416<F: Float>(t10409: F, t115003: F, t115016: F, t1253: F, t1466: F, t193: F, t24989: F, t25446: F, t25459: F, t28835: F, t28863: F, t28868: F, t29040: F, t29047: F, t29416: F, t31646: F, t31648: F, t31653: F, t4965: F, t5225: F, t55768: F, t6210: F, t6216: F, t6963: F, t6972: F, t7028: F, t880: F, t98359: F) -> (F,) {
    let t128718 = -24.0 * t55768 * t29047 - t6216 * t10409 * t25446 * t4965 / 27.0 + t25459 * t31653 / 9.0 + t29416 * t7028 / 3.0 - 2.0 / 3.0 * t29416 * t6972 + t6210 * t31648 + t1466 * t193 * t98359 * t31646 + t1466 * t193 * t24989 * t880 * t5225 + 4.0 / 27.0 * t115003 + t6963 * t28863 / 3.0 + t1466 * t193 * t29040 * t1253 / 3.0 + 4.0 / 27.0 * t115016 - 2.0 / 3.0 * t1466 * t193 * t28835 * t28868;
    (t128718,)
}
