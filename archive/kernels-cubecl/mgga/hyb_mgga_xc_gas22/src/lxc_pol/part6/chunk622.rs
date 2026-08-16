//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 622/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk622<F: Float>(t2869: F, t502: F, t1117: F, t1123: F, t1128: F, t1129: F, t1134: F, t1139: F, t1167: F, t1169: F, t2874: F, t2876: F, t2890: F, t2894: F, t2900: F, t2903: F, t2904: F, t2910: F, t2913: F, t2916: F, t2919: F, t2922: F, t2924: F, t2927: F, t2938: F, t510: F, t513: F, t518: F, t538: F) -> (F, F) {
    let t2940 = t502 * t2869;
    let t2943 = -F::cast_from(8.0_f64) * t1117 * t1128 * t1123 * t1129 - F::cast_from(72.0_f64) * t1134 * t1139 * t1123 * t1129 + F::cast_from(42.0_f64) * t518 * t2874 * t2876 + F::cast_from(2.0_f64) * t1117 * t2919 + F::cast_from(6.0_f64) * t1134 * t2913 + t1167 * t2890 - t1169 * t2894 + F::cast_from(6.0_f64) * t510 * t2900 + F::cast_from(30.0_f64) * t2903 * t2904 - F::cast_from(2.0_f64) * t510 * t2910 - F::cast_from(6.0_f64) * t518 * t2916 - F::cast_from(36.0_f64) * t2922 * t2924 - F::cast_from(4.0_f64) * t2927 * t2924 + t2938 * t538 + F::cast_from(2.0_f64) * t2940 * t513;
    (t2940, t2943)
}
