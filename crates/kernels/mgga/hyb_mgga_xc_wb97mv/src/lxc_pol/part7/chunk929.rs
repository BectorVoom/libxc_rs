//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 929/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk929<F: Float>(t3290: F, t549: F, t136: F, t1242: F, t6134: F, t1852: F, t3191: F, t3197: F, t3201: F, t8225: F, t39: F, t6526: F, t1232: F, t6528: F, t2039: F, t6536: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8630 = t549 * t3290;
    let t8632 = t136 * t8630 / 32.0;
    let t8637 = t6134 * t1242;
    let t8640 = 2.0 / 243.0 * t1852 * t3191;
    let t8642 = 2.0 / 81.0 * t1852 * t3197;
    let t8643 = t8225 * t3201;
    let t8645 = t6526 * t39;
    let t8646 = t6528 * t1232;
    let t8648 = t8645 * t8646 * t2039;
    let t8651 = t6536 * t1232;
    (t8630, t8632, t8637, t8640, t8642, t8643, t8645, t8648, t8651)
}
