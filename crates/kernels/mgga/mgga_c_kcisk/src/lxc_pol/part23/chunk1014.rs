//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1014/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1014<F: Float>(t13523: F, t13526: F, t13530: F, t13533: F, t13536: F, t20292: F, t20295: F, t20299: F, t20302: F, t20305: F, t20308: F, t20312: F, t20315: F, t20318: F, t20321: F, t20324: F, t20327: F) -> (F,) {
    let t20329 = -t13523 - 0.15829629629629629629e-1 * t13526 + 0.39574074074074074073e-2 * t13530 - 0.11872222222222222222e-1 * t13533 + 0.5936111111111111111e-2 * t13536 - 0.79148148148148148146e-2 * t20292 + 0.79148148148148148146e-2 * t20295 - t20299 + 0.13059444444444444444e0 * t20302 - 0.19787037037037037037e-1 * t20305 + 0.71233333333333333332e-1 * t20308 - 0.47488888888888888888e-1 * t20312 - 0.11872222222222222222e-1 * t20315 - 0.10685e0 * t20318 + 0.14246666666666666666e0 * t20321 + 0.35616666666666666666e-1 * t20324 - 0.35616666666666666666e-1 * t20327;
    (t20329,)
}
