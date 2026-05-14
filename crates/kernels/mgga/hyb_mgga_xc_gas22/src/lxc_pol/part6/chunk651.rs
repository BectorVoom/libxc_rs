//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 651/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk651<F: Float>(t143: F, t1270: F, t1285: F, t172: F, t187: F, t3226: F, t3227: F, t3267: F, t740: F, t759: F, t139: F, t214: F, t26: F, t1319: F, t550: F, t1318: F, t191: F) -> (F, F, F, F, F, F) {
    let t144 = 0.135e1 <= t143;
    let t3271 = piecewise3(t144, t3226, -8.0 / 3.0 * t1270 * t759 - 8.0 / 3.0 * t740 * t1285 - 8.0 / 3.0 * t172 * t3267 - 8.0 / 3.0 * t3227 * t187);
    let t3272 = t139 * t3271;
    let t3273 = t3272 * t214;
    let t3274 = t26 * t3273;
    let t3279 = t550 * t1319;
    let t3282 = t191 * t1318;
    (t3271, t3272, t3273, t3274, t3279, t3282)
}
