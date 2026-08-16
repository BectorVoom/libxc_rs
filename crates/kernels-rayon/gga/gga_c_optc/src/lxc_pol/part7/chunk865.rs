//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 865/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk865(t106: f64, t2688: f64, t2695: f64, t2818: f64, t335: f64, t3860: f64, t7931: f64, t7935: f64, t7939: f64, t7948: f64, t7949: f64, t7954: f64, t8263: f64, t908: f64, t956: f64) -> f64 {
    let t8267 = 0.27818116767324025134e1_f64 * t106 * t7931 * t335 - 0.83454350301972075402e1_f64 * t106 * t7935 * t956 + 0.16690870060394415081e2_f64 * t106 * t7939 * t2695 - 0.83454350301972075402e1_f64 * t106 * t2688 * t2818 - 0.1669087006039441508e2_f64 * t106 * t7948 * t7949 + 0.16690870060394415081e2_f64 * t3860 * t7954 - 0.27818116767324025134e1_f64 * t106 * t908 * t8263;
    t8267
}
