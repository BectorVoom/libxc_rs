//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1112/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1112<F: Float>(t21601: F, t378: F, t2593: F, t6996: F, t259: F, t461: F, t467: F, t495: F, t2723: F, t1047: F, t2712: F, t2713: F, t7449: F, t2674: F, t1110: F, t2635: F, t2639: F) -> (F, F, F, F, F, F, F, F, F) {
    let t21729 = t378 * t21601;
    let t21770 = t2593 * t6996;
    let t21832 = 24.0 * t461 * t467 * t259 * t495;
    let t21833 = t2723 * t2723;
    let t21836 = 6.0 * t2712 * t21833 * t1047;
    let t21837 = t2713 * t2713;
    let t21840 = 24.0 * t7449 * t21837 * t1047;
    let t21841 = t2674 * t2674;
    let t21845 = 0.51947577317044391277e2 * t1110 * t2635 * t21841 * t2639;
    (t21729, t21770, t21832, t21833, t21836, t21837, t21840, t21841, t21845)
}
