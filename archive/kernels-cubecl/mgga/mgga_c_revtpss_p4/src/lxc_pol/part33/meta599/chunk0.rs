//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2021/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2021<F: Float>(t136: F, t2457: F, t7307: F, t25944: F, t10073: F, t25937: F, t7274: F, t7282: F, t1955: F, t9656: F, t25904: F, t94634: F) -> (F, F, F, F, F) {
    let t94806 = t7307 * t136 * t2457;
    let t94807 = t25944 * t94806;
    let t94820 = t10073 * t7282 * t25937 * t7274;
    let t94823 = t1955 * t7282 * t9656;
    let t94842 = t25904 * t94634;
    (t94806, t94807, t94820, t94823, t94842)
}
