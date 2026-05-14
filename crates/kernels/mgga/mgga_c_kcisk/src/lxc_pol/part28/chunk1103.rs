//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1103/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1103<F: Float>(t12345: F, t12352: F, t18179: F, t18925: F, t2042: F, t2049: F, t25151: F, t25153: F, t25163: F, t25166: F, t25170: F, t25271: F, t2666: F, t5527: F, t5532: F, t7656: F, t7659: F, t7690: F, t802: F, t9262: F, t9291: F) -> (F,) {
    let t25273 = 2.0 * t12345 * t9262 - 6.0 * t12352 * t25163 - 2.0 * t18179 * t2666 + 4.0 * t18925 * t7659 - t2042 * t25271 - t2049 * t25153 + t25151 * t802 + 4.0 * t25166 * t5532 + 2.0 * t25170 * t5532 - t5527 * t9291 - 2.0 * t7656 * t7690;
    (t25273,)
}
