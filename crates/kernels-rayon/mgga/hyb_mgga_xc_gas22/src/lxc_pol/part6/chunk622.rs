//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 622/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk622(t2869: f64, t502: f64, t1117: f64, t1123: f64, t1128: f64, t1129: f64, t1134: f64, t1139: f64, t1167: f64, t1169: f64, t2874: f64, t2876: f64, t2890: f64, t2894: f64, t2900: f64, t2903: f64, t2904: f64, t2910: f64, t2913: f64, t2916: f64, t2919: f64, t2922: f64, t2924: f64, t2927: f64, t2938: f64, t510: f64, t513: f64, t518: f64, t538: f64) -> (f64, f64) {
    let t2940 = t502 * t2869;
    let t2943 = -8.0_f64 * t1117 * t1128 * t1123 * t1129 - 72.0_f64 * t1134 * t1139 * t1123 * t1129 + 42.0_f64 * t518 * t2874 * t2876 + 2.0_f64 * t1117 * t2919 + 6.0_f64 * t1134 * t2913 + t1167 * t2890 - t1169 * t2894 + 6.0_f64 * t510 * t2900 + 30.0_f64 * t2903 * t2904 - 2.0_f64 * t510 * t2910 - 6.0_f64 * t518 * t2916 - 36.0_f64 * t2922 * t2924 - 4.0_f64 * t2927 * t2924 + t2938 * t538 + 2.0_f64 * t2940 * t513;
    (t2940, t2943)
}
