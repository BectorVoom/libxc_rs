//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2663/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2663<F: Float>(t1307: F, t20563: F, t12211: F, t20516: F, t20501: F, t3726: F, t54042: F, t6390: F, t20479: F, t3866: F, t16336: F, t6427: F) -> (F, F, F, F, F, F) {
    let t74389 = t20563 * t1307;
    let t74393 = t12211 * t20516;
    let t74395 = t3726 * t20501;
    let t74401 = t54042 * t6390;
    let t74403 = t3866 * t20479;
    let t74405 = t16336 * t6427;
    (t74389, t74393, t74395, t74401, t74403, t74405)
}
