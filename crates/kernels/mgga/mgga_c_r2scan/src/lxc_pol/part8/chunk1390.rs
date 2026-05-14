//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1390/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1390<F: Float>(t19069: F, t19341: F, t23781: F, t23798: F, t26938: F, t26945: F, t28976: F, t28982: F, t32116: F, t32217: F, t32218: F, t32219: F, t765: F, t19388: F, t19394: F, t19405: F, t28989: F, t28991: F, t28993: F, t28995: F, t32228: F, t32963: F, t32965: F, t32967: F) -> (F, F) {
    let t33762 = t32217 - t32218 - t19069 + 0.857292e-1 * t28976 + t23781 + 0.675260332e-1 * t765 * t32116 + t19341 + 0.12154685976e1 * t26938 + t32219 - t23798 + t26945 + 0.857292e-1 * t28982;
    let t33770 = t32228 + t19388 + t19394 + t32963 - t19405 - t32965 + t32967 - 0.2025780996e0 * t28989 - 0.2025780996e0 * t28991 - 0.2025780996e0 * t28993 - 0.4051561992e0 * t28995;
    (t33762, t33770)
}
