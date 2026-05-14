//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 958/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk958<F: Float>(t12378: F, t448: F, t300: F, t12295: F, t12292: F, t12297: F, t12299: F, t12301: F, t12303: F, t12307: F, t12310: F, t12314: F, t12317: F, t12320: F, t422: F, t1151: F, t3427: F) -> (F, F, F, F, F) {
    let t12379 = t12378 * t448;
    let t12381 = 0.19751673498613801407e-1 * t300 * t12379;
    let t12382 = 0.55403703703703703703e-1 * t12295;
    let t12393 = -t12382 + 0.23744444444444444444e-1 * t12297 + 0.11872222222222222222e-1 * t12299 - 0.35616666666666666666e-1 * t12301 - 0.17808333333333333333e-1 * t12303 + 0.19787037037037037037e-1 * t12307 - 0.71233333333333333332e-1 * t12310 - 0.35616666666666666666e-1 * t12292 + 0.10685e0 * t12314 + 0.10685e0 * t12317 + 0.17808333333333333333e-1 * t12320;
    let t12395 = 0.621814e-1 * t12393 * t422;
    let t12397 = 0.53272592592592592592e-1 * t12295;
    let t12408 = -t12397 + 0.2283111111111111111e-1 * t12297 + 0.11415555555555555555e-1 * t12299 - 0.34246666666666666665e-1 * t12301 - 0.17123333333333333333e-1 * t12303 + 0.19025925925925925925e-1 * t12307 - 0.68493333333333333331e-1 * t12310 - 0.34246666666666666665e-1 * t12292 + 0.10274e0 * t12314 + 0.10274e0 * t12317 + 0.17123333333333333333e-1 * t12320;
    let t12411 = t1151 * t3427;
    (t12379, t12381, t12395, t12408, t12411)
}
