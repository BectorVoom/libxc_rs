//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 157/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk157<F: Float>(t673: F, t702: F, t140: F, t479: F, t709: F, t725: F, t716: F) -> (F, F, F) {
    let t728 = t673 * t702;
    let t732 = 0.619125e-2 * t725 * t709 - 0.39796666666666666666e-1 * t140 * t479 * t728;
    let t733 = t732 * t716;
    (t728, t732, t733)
}
