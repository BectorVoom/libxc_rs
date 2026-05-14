//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 780/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk780<F: Float>(t2512: F, t819: F, t2520: F, t817: F, t7342: F, t837: F, t2492: F, t805: F, t2494: F, t818: F, t2493: F, t2495: F, t2518: F, t252: F, t2530: F, t2537: F, t7621: F, t7626: F, t7628: F, t7631: F, t7666: F, t7688: F, t7694: F, t7710: F, t7726: F, t7727: F, t7731: F) -> (F, F, F, F, F, F, F) {
    let t7734 = t819 * t2512;
    let t7738 = t2512 * t2520 * t817;
    let t7741 = t7342 * t837;
    let t7744 = t805 * t2492;
    let t7747 = t2494 * t817;
    let t7748 = t7747 * t818;
    let t7752 = -0.3109e-1 * t7710 * t252 - t7688 + t7694 + t7726 - 0.35089340384731224426e1 * t2530 * t7727 + 0.51947267698127589897e2 * t2537 * t7731 - 6.0 * t2493 * t7734 + 0.96494049533612093922e2 * t2518 * t7738 + 0.35089340384731224426e1 * t2537 * t7741 - 6.0 * t7744 * t2495 + 6.0 * t2518 * t7748 - 0.19751789702565206229e-1 * t7621 - t7626 - t7628 - t7631 - t7666;
    (t7734, t7738, t7741, t7744, t7747, t7748, t7752)
}
