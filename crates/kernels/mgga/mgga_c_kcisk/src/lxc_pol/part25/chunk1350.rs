//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1350/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1350<F: Float>(t117580: F, t5219: F, t63008: F, t9699: F, t10028: F, t117552: F, t117556: F, t117557: F, t117560: F, t117565: F, t117568: F, t117574: F, t18179: F, t18925: F, t18928: F, t2049: F, t33132: F, t33153: F, t33156: F, t33306: F, t33312: F, t48513: F, t5532: F, t5533: F, t64998: F, t65015: F, t7690: F, t9763: F, t9772: F) -> (F, F, F) {
    let t117582 = 2.0 * t117580 * t5219;
    let t117586 = 4.0 * t63008 * t9699;
    let t117591 = 4.0 * t5532 * t7690 * t9772 + 2.0 * t10028 * t48513 + 2.0 * t117552 * t5533 - 2.0 * t117560 * t2049 - 2.0 * t18179 * t9772 + 4.0 * t18925 * t33132 + 2.0 * t18925 * t33156 + 2.0 * t18928 * t33153 - 2.0 * t33306 * t7690 - 6.0 * t33312 * t65015 + 4.0 * t64998 * t9763 - t117556 + t117557 + t117565 + t117568 + t117574 - t117582 - t117586;
    (t117582, t117586, t117591)
}
