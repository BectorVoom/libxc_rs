//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 816/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk816<F: Float>(t106: F, t2688: F, t2695: F, t2818: F, t335: F, t3860: F, t7931: F, t7935: F, t7939: F, t7948: F, t7949: F, t7954: F, t8263: F, t908: F, t956: F, t4044: F, t7178: F) -> (F, F) {
    let t8267 = 0.27818116767324025134e1 * t106 * t7931 * t335 - 0.83454350301972075402e1 * t106 * t7935 * t956 + 0.16690870060394415081e2 * t106 * t7939 * t2695 - 0.83454350301972075402e1 * t106 * t2688 * t2818 - 0.1669087006039441508e2 * t106 * t7948 * t7949 + 0.16690870060394415081e2 * t3860 * t7954 - 0.27818116767324025134e1 * t106 * t908 * t8263;
    let t8272 = t4044 * t7178;
    (t8267, t8272)
}
