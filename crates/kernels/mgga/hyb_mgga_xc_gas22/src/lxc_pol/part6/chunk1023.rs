//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1023/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1023<F: Float>(t3997: F, t720: F, t157: F, t4014: F, t724: F, t160: F, t728: F, t1890: F, t4002: F, t3925: F, t6291: F, t675: F, t8296: F, t6299: F, t3171: F, t3: F, t3172: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t10262 = t720 * t3997;
    let t10267 = t157 * t4014;
    let t10270 = t724 * t3997;
    let t10275 = t160 * t4014;
    let t10278 = t728 * t3997;
    let t10286 = t1890 * t4002;
    let t10288 = t6291 * t3925;
    let t10290 = t8296 * t10288 * t675;
    let t10293 = t6299 * t3925;
    let t10295 = t3171 * t10293 * t675;
    let t10299 = t3171 * t3172 * t3;
    (t10262, t10267, t10270, t10275, t10278, t10286, t10288, t10290, t10293, t10295, t10299)
}
