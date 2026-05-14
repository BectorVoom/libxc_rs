//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1064/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1064<F: Float>(t24345: F, t6759: F, t22484: F, t7378: F, t22488: F, t22501: F, t22506: F, t7370: F, t140: F, t18081: F, t18092: F, t22928: F, t24320: F, t24324: F, t24326: F, t24332: F, t24335: F, t24338: F, t24342: F, t2521: F, t2543: F, t479: F, t5231: F, t6278: F, t7039: F, t7340: F) -> (F,) {
    let t24346 = t24345 * t6759;
    let t24349 = t7378 * t22484;
    let t24352 = t7378 * t22488;
    let t24355 = t7378 * t22501;
    let t24358 = t7370 * t22506;
    let t24361 = 0.1857375e-1 * t2543 * t7039 - 0.123825e-1 * t7340 * t2521 - 0.29479012345679012345e-1 * t24320 - 0.35374814814814814815e-1 * t18092 - 0.26531111111111111111e-1 * t24324 - 0.39796666666666666666e-1 * t140 * t479 * t24326 - 0.9286875e-2 * t5231 * t22928 - 0.11791604938271604938e0 * t6278 * t24332 + 0.17687407407407407407e0 * t18081 * t24335 + 0.26531111111111111111e0 * t6278 * t24338 + 0.10612444444444444444e0 * t6278 * t24342 - 0.88437037037037037037e-1 * t6278 * t24346 - 0.15918666666666666667e0 * t6278 * t24349 - 0.21224888888888888889e0 * t18081 * t24352 + 0.53062222222222222222e-1 * t6278 * t24355 - 0.44218518518518518518e-1 * t6278 * t24358;
    (t24361,)
}
