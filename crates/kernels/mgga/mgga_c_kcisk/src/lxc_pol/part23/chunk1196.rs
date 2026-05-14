//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1196/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1196<F: Float>(t32625: F, t32677: F, t1140: F, t1147: F, t15713: F, t15716: F, t2705: F, t289: F, t31998: F, t32560: F, t32579: F, t32583: F, t32584: F, t32588: F, t3437: F, t3442: F, t3460: F, t9392: F, t9395: F, t9404: F) -> (F, F) {
    let t32678 = t32625 + t32677;
    let t32682 = -t1140 * t32579 - 2.0 * t1147 * t32560 - t15713 * t2705 + 4.0 * t15716 * t9395 + t289 * t32678 + 4.0 * t32584 * t3442 - 2.0 * t3437 * t9404 - t3460 * t9392 - t31998 + t32583 + t32588;
    (t32678, t32682)
}
