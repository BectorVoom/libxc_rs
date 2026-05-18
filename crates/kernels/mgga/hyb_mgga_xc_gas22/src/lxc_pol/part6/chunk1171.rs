//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1171/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1171<F: Float>(t2249: F, t2271: F, t262: F, t20624: F, t20688: F, t2274: F, t7147: F, t944: F, t2519: F, t347: F, t2522: F, t2470: F, t2477: F) -> (F, F, F, F, F, F, F, F, F) {
    let t21053 = t262 / t2271 / t2249;
    let t21057 = F::new(0.5356037037037037037e1) * t20624;
    let t21071 = F::new(0.16979925925925925926e1) * t20688;
    let t21087 = t2271 * t2271;
    let t21089 = t262 / t21087;
    let t21090 = t2274 * t2274;
    let t21091 = F::new(1.0) / t21090;
    let t21366 = t944 * t7147;
    let t21369 = t2519 * t2519;
    let t21371 = t347 / t21369;
    let t21373 = t2522 * t2522;
    let t21374 = F::new(1.0) / t21373;
    let t21378 = t2470 * t2477;
    (t21053, t21057, t21071, t21089, t21091, t21366, t21371, t21374, t21378)
}
