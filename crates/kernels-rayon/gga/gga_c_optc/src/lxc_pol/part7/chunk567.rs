//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 567/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk567(t106: f64, t2684: f64, t2688: f64, t2694: f64, t2695: f64, t2818: f64, t335: f64, t908: f64, t956: f64, t1015: f64, t2421: f64, t2430: f64, t2433: f64, t2438: f64, t2443: f64, t2451: f64, t2470: f64, t2479: f64, t2542: f64, t2544: f64, t2551: f64, t2555: f64, t2561: f64, t2563: f64, t2566: f64, t2569: f64, t277: f64, t95: f64, t962: f64, t999: f64) -> (f64, f64) {
    let t2822 = 0.27818116767324025134e1_f64 * t106 * t2684 * t335 - 0.55636233534648050268e1_f64 * t106 * t2688 * t956 + 0.55636233534648050268e1_f64 * t106 * t2694 * t2695 - 0.27818116767324025134e1_f64 * t106 * t908 * t2818;
    let t2827 = t2421 + t2430 + 100.0_f64 / 81.0_f64 * t2433 * t2438 - t2443 - t2451 - t2470 - t2479 + t2542 + t999 * t2544 / 6.0_f64 + 2.0_f64 / 9.0_f64 * t999 * t2551 + 100.0_f64 / 27.0_f64 * t2555 * t1015 + t2561 - t999 * t2563 / 3.0_f64 - 0.25844881434903430496e-2_f64 * t95 * t277 * t2566 * t2569 + 0.25844881434903430496e-2_f64 * t95 * t277 * t2822 * t962;
    (t2822, t2827)
}
