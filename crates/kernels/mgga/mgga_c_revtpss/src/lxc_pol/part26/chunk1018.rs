//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1018/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1018<F: Float>(t1955: F, t7282: F, t9656: F, t1398: F, t4077: F, t543: F, t281: F, t555: F, t93238: F, t1444: F, t4057: F, t1426: F, t94609: F, t7063: F, t25877: F, t94801: F) -> (F, F, F, F, F, F, F) {
    let t94823 = t1955 * t7282 * t9656;
    let t94825 = t4077 * t1398 * t543;
    let t94849 = t281 * t93238 * t555;
    let t94868 = t4057 * t1444;
    let t94878 = t94609 * t1426;
    let t94879 = t7063 * t94878;
    let t94886 = t94801 * t25877;
    (t94823, t94825, t94849, t94868, t94878, t94879, t94886)
}
