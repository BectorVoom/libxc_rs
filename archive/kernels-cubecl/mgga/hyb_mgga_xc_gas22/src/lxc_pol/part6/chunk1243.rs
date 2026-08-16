//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1243/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1243<F: Float>(t1056: F, t3466: F, t2520: F, t3474: F, t1396: F, t7147: F, t2477: F, t7074: F, t9266: F, t948: F, t260: F, t9031: F) -> (F, F, F, F, F, F, F) {
    let t25245 = F::cast_from(8.0_f64) * t3466 * t1056;
    let t25257 = t3474 * t2520;
    let t25262 = t1396 * t7147;
    let t25267 = t3474 * t2477;
    let t25270 = t1396 * t7074;
    let t25273 = t9266 * t948;
    let t25276 = t260 * t9031;
    (t25245, t25257, t25262, t25267, t25270, t25273, t25276)
}
