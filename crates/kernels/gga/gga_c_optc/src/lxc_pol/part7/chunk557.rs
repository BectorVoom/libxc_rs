//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 557/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk557<F: Float>(t106: F, t2684: F, t2688: F, t2694: F, t2695: F, t2818: F, t335: F, t908: F, t956: F, t1015: F, t2421: F, t2430: F, t2433: F, t2438: F, t2443: F, t2451: F, t2470: F, t2479: F, t2542: F, t2544: F, t2551: F, t2555: F, t2561: F, t2563: F, t2566: F, t2569: F, t277: F, t95: F, t962: F, t999: F) -> (F, F) {
    let t2822 = 0.27818116767324025134e1 * t106 * t2684 * t335 - 0.55636233534648050268e1 * t106 * t2688 * t956 + 0.55636233534648050268e1 * t106 * t2694 * t2695 - 0.27818116767324025134e1 * t106 * t908 * t2818;
    let t2827 = t2421 + t2430 + 100.0 / 81.0 * t2433 * t2438 - t2443 - t2451 - t2470 - t2479 + t2542 + t999 * t2544 / 6.0 + 2.0 / 9.0 * t999 * t2551 + 100.0 / 27.0 * t2555 * t1015 + t2561 - t999 * t2563 / 3.0 - 0.25844881434903430496e-2 * t95 * t277 * t2566 * t2569 + 0.25844881434903430496e-2 * t95 * t277 * t2822 * t962;
    (t2822, t2827)
}
