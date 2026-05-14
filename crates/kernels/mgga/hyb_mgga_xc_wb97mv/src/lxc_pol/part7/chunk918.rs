//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 918/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk918<F: Float>(t556: F, t8473: F, t3005: F, t554: F, t1993: F, t1996: F, t3004: F, t557: F, t6383: F, t6384: F, t6386: F, t6389: F, t6391: F, t6393: F, t6396: F, t8451: F, t8455: F, t8459: F, t8463: F, t8469: F, t8472: F) -> (F, F, F) {
    let t8474 = t8473 * t556;
    let t8476 = t554 * t8474 * t3005;
    let t8478 = -t6383 + t6384 / 48.0 - t6386 / 64.0 + t6389 / 48.0 - t6391 / 32.0 - t6393 / 32.0 + t6396 / 48.0 - t1993 * t1996 * t8451 / 48.0 - t554 * t3004 * t8455 / 16.0 - t554 * t557 * t8459 / 64.0 - t554 * t3004 * t8463 / 16.0 - t8469 - t8472 + 7.0 / 96.0 * t8476;
    (t8474, t8476, t8478)
}
