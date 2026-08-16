//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3002/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3002<F: Float>(t10995: F, t122: F, t14982: F, t2466: F, t10777: F, t10779: F, t1548: F, t2646: F, t10868: F, t820: F, t844: F, t14896: F) -> (F, F, F, F) {
    let t50259 = t10995 * t14982 * t122 * t2466;
    let t50292 = t10777 * t10779 * t1548 * t2646;
    let t50295 = t820 * t10868 * t844;
    let t50296 = t50295 * t14896;
    (t50259, t50292, t50295, t50296)
}
