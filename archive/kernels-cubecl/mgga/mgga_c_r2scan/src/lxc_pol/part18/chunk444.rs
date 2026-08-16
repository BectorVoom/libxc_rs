//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 444/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk444<F: Float>(t2168: F, t565: F, t110: F, t1598: F, t524: F, t531: F, t108: F, t144: F, t543: F) -> (F, F, F, F, F) {
    let t2169 = t565 * t2168;
    let t2176 = t1598 * t110;
    let t2177 = t524 * t2176;
    let t2178 = t2177 * t531;
    let t2182 = t108 / t543 / t144;
    (t2169, t2176, t2177, t2178, t2182)
}
