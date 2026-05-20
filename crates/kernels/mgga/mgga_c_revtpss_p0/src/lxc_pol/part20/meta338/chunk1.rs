//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1263/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1263<F: Float>(t2719: F, t820: F, t844: F, t2482: F, t814: F, t11509: F, t2988: F, t4900: F, t999: F, t4894: F, t245: F, t4890: F) -> (F, F, F, F, F, F) {
    let t14923 = t820 * t2719 * t844;
    let t14931 = t2482 * t2719 * t814;
    let t15542 = t11509 * t2988;
    let t15604 = t4900 * t999;
    let t15609 = t4894 * t999;
    let t15687 = t4890 * t245;
    (t14923, t14931, t15542, t15604, t15609, t15687)
}
