//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 491/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk491<F: Float>(t264: F, t3374: F, t3376: F, t2925: F, t67: F, t10: F, t1102: F, t119: F, t142: F, t260: F, t261: F, t116: F, t1111: F, t1118: F, t20: F, t918: F, t268: F) -> (F, F, F, F, F, F, F) {
    let t265 = t264 < -0.66725e-1;
    let t3377 = t3374 * t3376;
    let t3380 = t67 * t2925;
    let t3391 = piecewise3(t265, 0.0, 10.0 / 9.0 * t260 * t3380 * t10 - 20.0 / 27.0 * t260 * t1102 * t142 + 40.0 / 81.0 * t260 * t261 * t119);
    let t3392 = t3391 * t116;
    let t3399 = t1111 * t1118;
    let t3405 = t918 * t20;
    let t3406 = t268 * t3405;
    (t3377, t3380, t3391, t3392, t3399, t3405, t3406)
}
