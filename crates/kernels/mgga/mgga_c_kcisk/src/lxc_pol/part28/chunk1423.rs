//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1423/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1423<F: Float>(t117866: F, t18775: F, t34427: F, t112937: F, t117740: F, t117900: F, t117906: F, t117913: F, t117925: F, t117927: F, t117967: F, t118412: F, t118473: F, t118474: F, t121329: F, t122554: F, t22289: F, t22294: F, t2803: F, t2807: F, t33196: F, t34419: F, t34560: F, t34561: F, t35416: F, t73230: F, t79: F, t9740: F) -> (F, F) {
    let t122694 = t117866 * t18775 * t34427;
    let t122713 = t117900 - t117906 + t117913 + 0.92592592592592592593e-2 * t117925 - 0.52083333333333333333e-2 * t73230 * t79 * t2803 * t2807 - 0.15476481481481481481e-2 * t121329 + t117927 - 0.116403125e-2 * t112937 * t35416 + 0.120625e-1 * t33196 * t122694 + 0.23280625e-2 * t34419 * t122694 + 0.13888888888888888889e-1 * t9740 * t34560 * t118412 * t22289 + 0.89351851851851851851e-3 * t33196 * t122554 - 0.54012345679012345679e-2 * t9740 * t118473 * t118474 * t22289 + 0.92592592592592592592e-2 * t9740 * t117740 * t34561 * t22294 + t117967;
    (t122694, t122713)
}
