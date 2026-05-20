//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3004/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3004<F: Float>(t10777: F, t10779: F, t14671: F, t14872: F, t10811: F, t14682: F, t14804: F, t14923: F, t4457: F, t837: F, t14853: F, t2652: F) -> (F, F, F, F, F) {
    let t50325 = t10777 * t10779 * t14671 * t14872;
    let t50328 = t10811 * t14682;
    let t50347 = t14923 * t14804;
    let t50351 = t10777 * t10779 * t4457 * t837;
    let t50353 = t2652 * t14853;
    (t50325, t50328, t50347, t50351, t50353)
}
