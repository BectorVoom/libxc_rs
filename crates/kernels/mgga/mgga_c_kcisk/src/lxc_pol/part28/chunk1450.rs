//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1450/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1450<F: Float>(t2041: F, t35521: F, t1957: F, t35337: F, t5218: F, t24495: F, t9696: F, t10039: F, t112139: F, t122374: F, t122375: F, t122376: F, t122379: F, t122380: F, t122382: F, t18179: F, t18925: F, t2049: F, t25271: F, t2815: F, t34377: F, t34386: F, t34612: F, t35374: F, t5527: F, t5532: F, t65015: F, t65181: F, t7690: F, t9262: F, t9760: F) -> (F, F, F) {
    let t123453 = t35521 * t2041;
    let t123464 = 2.0 * t5218 * t35337 * t1957;
    let t123465 = t9696 * t24495;
    let t123466 = 4.0 * t10039 * t5532 * t7690 - 2.0 * t10039 * t18179 + 2.0 * t112139 * t9262 - t123453 * t2049 + 4.0 * t18925 * t34612 - t25271 * t9760 - t2815 * t65181 - 2.0 * t34377 * t7690 - 12.0 * t34386 * t65015 - t35374 * t5527 + t122374 + t122375 + t122376 + t122379 + t122380 + t122382 - t123464 + t123465;
    (t123464, t123465, t123466)
}
