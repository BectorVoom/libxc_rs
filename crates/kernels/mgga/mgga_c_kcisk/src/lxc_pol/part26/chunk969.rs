//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 969/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk969<F: Float>(t1266: F, t1275: F, t26289: F, t4126: F, t7993: F, t6125: F, t13523: F, t13526: F, t20292: F, t20295: F, t20299: F, t20302: F, t26138: F, t26141: F, t26144: F, t26147: F, t26150: F, t26153: F, t26156: F, t26159: F, t26162: F, t26165: F, t26168: F) -> (F, F, F) {
    let t26291 = t1266 * t26289 * t1275;
    let t26302 = t4126 * t7993;
    let t26303 = t26302 * t6125;
    let t26321 = -t13523 - 0.79148148148148148147e-2 * t13526 - 0.15829629629629629629e-1 * t20292 + 0.79148148148148148147e-2 * t20295 - t20299 + 0.23744444444444444444e-1 * t20302 + 0.39574074074074074073e-2 * t26138 - 0.19787037037037037037e-1 * t26141 + 0.71233333333333333332e-1 * t26144 - 0.47488888888888888888e-1 * t26147 - 0.11872222222222222222e-1 * t26150 - 0.10685e0 * t26153 + 0.14246666666666666666e0 * t26156 + 0.5936111111111111111e-2 * t26159 - 0.11872222222222222222e-1 * t26162 + 0.35616666666666666666e-1 * t26165 - 0.17808333333333333333e-1 * t26168;
    (t26291, t26303, t26321)
}
