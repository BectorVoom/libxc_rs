//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 835/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk835<F: Float>(t6881: F, t6888: F, t7126: F, t7129: F, t7132: F, t8649: F, t8651: F, t8652: F, t881: F, t9056: F, t9787: F, t9791: F, t9816: F, t2266: F, t2267: F, t3016: F) -> (F, F) {
    let t9818 = -0.2363e1 * t881 * t9056 - t9787 - t8649 + t8651 + t6881 - t9791 - 0.2363e1 * t6888 - t7126 - t8652 - 0.4726e1 * t7129 - t7132 - 0.4726e1 * t9816;
    let t9824 = t2266 * t2267 * t3016;
    (t9818, t9824)
}
