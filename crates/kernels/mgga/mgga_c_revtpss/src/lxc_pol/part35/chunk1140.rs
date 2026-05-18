//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1140/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1140<F: Float>(t530: F, t8107: F, t136: F, t2457: F, t8103: F, t25944: F, t10073: F, t1903: F, t2102: F, t25929: F, t28837: F, t3920: F) -> (F, F, F, F, F) {
    let t102015 = t530 * t8107;
    let t102100 = t8103 * t136 * t2457;
    let t102101 = t25944 * t102100;
    let t102120 = t10073 * t25929 * t2102 * t1903;
    let t102122 = t28837 * t3920;
    (t102015, t102100, t102101, t102120, t102122)
}
