//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1003/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1003<F: Float>(t545: F, t94667: F, t25875: F, t25894: F, t1426: F, t9990: F, t7282: F, t9646: F, t93139: F, t2028: F, t3999: F, t25877: F, t94382: F, t1955: F, t9656: F, t281: F, t555: F, t93238: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t94668 = t94667 * t545;
    let t94669 = t25875 * t94668;
    let t94674 = t25894 * t94668;
    let t94683 = t1426 * t9990;
    let t94696 = t9646 * t7282;
    let t94701 = t93139 * t7282;
    let t94762 = t2028 * t3999;
    let t94763 = t25875 * t94762;
    let t94768 = t25894 * t94762;
    let t94771 = t94382 * t25877;
    let t94823 = t1955 * t7282 * t9656;
    let t94849 = t281 * t93238 * t555;
    (t94669, t94674, t94683, t94696, t94701, t94763, t94768, t94771, t94823, t94849)
}
