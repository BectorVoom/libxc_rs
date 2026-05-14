//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1074/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1074<F: Float>(t2466: F, t7504: F, t2473: F, t7501: F, t845: F, t2248: F, t7207: F, t24223: F, t24225: F, t24228: F, t24230: F, t24233: F, t24299: F, t24308: F, t24337: F, t24339: F) -> (F, F) {
    let t24341 = t7504 * t2466;
    let t24344 = 0.61523382126046769581e4 * t845 * t7501 * t2473 * t24341;
    let t24345 = t2248 * t7207;
    let t24347 = t24223 + t24225 + t24228 + t24230 + t24233 + t24299 + t24308 + t24337 + t24339 - t24344 + 200.0 / 9.0 * t24345;
    (t24344, t24347)
}
