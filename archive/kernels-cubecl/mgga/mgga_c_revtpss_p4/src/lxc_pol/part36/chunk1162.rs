//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1162/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1162<F: Float>(t1209: F, t29135: F, t2142: F, t5219: F, t3801: F, t8220: F, t1479: F, t60: F, t2122: F, t28150: F, t13272: F, t7565: F) -> (F, F, F, F, F, F) {
    let t29275 = t1209 * t29135;
    let t29304 = t5219 * t2142;
    let t29317 = t8220 * t3801;
    let t29355 = t1479 * t60;
    let t29380 = t2122 * t28150;
    let t29388 = t13272 * t7565;
    (t29275, t29304, t29317, t29355, t29380, t29388)
}
