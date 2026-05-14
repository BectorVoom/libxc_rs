//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1204/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1204<F: Float>(t26237: F, t26240: F, t26242: F, t26245: F, t26251: F, t26455: F, t26457: F, t26459: F, t26463: F, t26467: F, t26470: F, t26472: F, t2995: F, t3012: F, t3018: F, t1057: F, t2993: F, t8679: F) -> (F, F, F) {
    let t26473 = t26237 + t26240 + t26242 + t26245 - t26251 + t26455 - t26457 + t26459 + t26463 + t26467 - t26470 - t26472;
    let t26476 = 36.0 * t3018 * t2995 * t3012;
    let t26479 = 8.0 * t2993 * t1057 * t8679;
    (t26473, t26476, t26479)
}
