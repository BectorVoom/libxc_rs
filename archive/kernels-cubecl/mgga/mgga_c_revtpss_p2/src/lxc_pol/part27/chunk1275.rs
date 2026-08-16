//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1275/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1275<F: Float>(t7284: F, t94810: F, t25878: F, t94597: F, t10073: F, t25937: F, t7274: F, t7282: F, t1955: F, t9656: F, t1398: F, t4077: F, t543: F) -> (F, F, F, F, F) {
    let t94811 = t7284 * t94810;
    let t94813 = t25878 * t94597;
    let t94820 = t10073 * t7282 * t25937 * t7274;
    let t94823 = t1955 * t7282 * t9656;
    let t94825 = t4077 * t1398 * t543;
    (t94811, t94813, t94820, t94823, t94825)
}
