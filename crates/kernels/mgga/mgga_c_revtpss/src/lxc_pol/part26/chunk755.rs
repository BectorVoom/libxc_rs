//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 755/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk755<F: Float>(t9736: F, t9738: F, t1369: F, t2699: F, t1372: F, t3943: F, t794: F, t3946: F, t1412: F, t159: F, t216: F, t124: F, t800: F, t9400: F) -> (F, F, F, F, F) {
    let t9739 = t9736 * t9738;
    let t9741 = t2699 * t1369;
    let t9742 = t9741 * t1372;
    let t9744 = t794 * t3943;
    let t9745 = t9744 * t3946;
    let t9747 = t159 * t1412;
    let t9748 = t216 * t9747;
    let t9750 = t800 * t124 * t9400;
    (t9739, t9742, t9745, t9748, t9750)
}
