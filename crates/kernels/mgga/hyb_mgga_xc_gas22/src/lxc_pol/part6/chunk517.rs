//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 517/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk517<F: Float>(t2485: F, t2496: F, t2490: F, t957: F, t2213: F, t238: F, t353: F, t801: F, t963: F) -> (F, F, F, F, F) {
    let t2497 = t2496 * t2485;
    let t2499 = t957 * t2490;
    let t2502 = t238 * t2213 * t353;
    let t2503 = 0.13692777777777777778e0 * t2502;
    let t2505 = t238 * t801 * t963;
    (t2497, t2499, t2502, t2503, t2505)
}
