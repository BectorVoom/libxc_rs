//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 431/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk431<F: Float>(t1140: F, t1147: F, t289: F, t3180: F, t3183: F, t3189: F, t3275: F, t3435: F, t3437: F, t3442: F, t3443: F, t3460: F, t233: F, t1337: F, t453: F, t1336: F, t140: F) -> (F, F, F, F) {
    let t3462 = -t1140 * t3460 - 2.0 * t1147 * t3437 + t289 * t3435 + 2.0 * t3442 * t3443 - t3180 + t3183 - t3189 + t3275;
    let t3463 = t233 * t3462;
    let t3480 = t1337 * t453;
    let t3482 = t140 * t1336 * t3480;
    (t3462, t3463, t3480, t3482)
}
