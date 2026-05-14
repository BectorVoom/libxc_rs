//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1066/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1066<F: Float>(t1919: F, t1920: F, t22591: F, t11853: F, t11885: F, t1470: F, t18089: F, t18132: F, t22659: F, t22663: F, t22672: F, t24364: F, t24367: F, t24374: F, t24376: F, t24380: F, t24388: F, t24392: F, t24396: F, t3077: F, t5231: F, t7035: F, t709: F, t7349: F, t7360: F) -> (F,) {
    let t24400 = t1919 * t1920 * t22591;
    let t24403 = 0.371475e-1 * t5231 * t24364 - 0.9286875e-2 * t7349 * t24367 - 0.9286875e-2 * t5231 * t22659 + 0.123825e-1 * t7360 * t22663 + 0.35374814814814814815e-1 * t24374 - 0.17687407407407407407e-1 * t24376 - 0.1857375e-1 * t18089 * t7035 + 0.619125e-2 * t24380 * t709 + 0.58958024691358024691e-2 * t11853 - 0.371475e-1 * t7360 * t22672 + 0.11791604938271604938e-1 * t18132 - t11885 + 0.10612444444444444444e0 * t3077 * t24388 + 0.53062222222222222222e-1 * t1470 * t24392 - 0.26531111111111111111e-1 * t1470 * t24396 - 0.26531111111111111111e-1 * t1470 * t24400;
    (t24403,)
}
