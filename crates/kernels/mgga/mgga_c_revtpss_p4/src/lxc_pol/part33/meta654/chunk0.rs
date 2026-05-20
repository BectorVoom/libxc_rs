//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2105/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2105<F: Float>(t2143: F, t3566: F, t17306: F, t2142: F, t3556: F, t8945: F, t12640: F, t7635: F, t29313: F, t3801: F, t12587: F, t8220: F) -> (F, F, F, F, F, F) {
    let t105576 = t3566 * t2143;
    let t105579 = t17306 * t2142;
    let t105598 = t3556 * t8945;
    let t105644 = t12640 * t7635;
    let t105665 = t29313 * t3801;
    let t105669 = t8220 * t12587;
    (t105576, t105579, t105598, t105644, t105665, t105669)
}
