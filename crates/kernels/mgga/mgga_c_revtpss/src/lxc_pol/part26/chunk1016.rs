//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1016/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1016<F: Float>(t1032: F, t9656: F, t545: F, t25875: F, t25894: F, t1426: F, t9990: F, t7282: F, t9646: F, t93139: F, t1955: F, t25920: F, t4075: F, t1398: F, t4131: F, t543: F) -> (F, F, F, F, F, F, F) {
    let t94667 = t1032 * t9656;
    let t94668 = t94667 * t545;
    let t94669 = t25875 * t94668;
    let t94674 = t25894 * t94668;
    let t94683 = t1426 * t9990;
    let t94696 = t9646 * t7282;
    let t94701 = t93139 * t7282;
    let t94705 = t1955 * t25920 * t4075;
    let t94721 = t4131 * t1398 * t543;
    (t94669, t94674, t94683, t94696, t94701, t94705, t94721)
}
