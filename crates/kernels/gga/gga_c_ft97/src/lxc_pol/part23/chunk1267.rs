//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1267/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1267<F: Float>(t124200: F, t24432: F, t6118: F, t108255: F, t1091: F, t2354: F, t123883: F, t24438: F, t122007: F, t27762: F, t109434: F, t109476: F, t110235: F, t110238: F, t110245: F, t97232: F, t97244: F, t97412: F) -> (F, F, F, F, F) {
    let t124311 = t6118 * t24432 * t124200;
    let t124316 = t6118 * t2354 * t108255 * t1091;
    let t124322 = t6118 * t24438 * t123883;
    let t124325 = t6118 * t27762 * t122007;
    let t124327 = -2.0 / 9.0 * t124311 + t97232 / 27.0 + t124316 / 9.0 - 8.0 / 27.0 * t109434 + t110235 - 4.0 / 27.0 * t97244 + t97412 + t110238 + t110245 - 4.0 / 27.0 * t109476 - 2.0 / 9.0 * t124322 + 2.0 / 27.0 * t124325;
    (t124311, t124316, t124322, t124325, t124327)
}
