//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 854/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk854<F: Float>(t6173: F, t954: F, t2970: F, t6157: F, t2974: F, t4571: F, t6094: F, t6098: F, t6102: F, t324: F, t1633: F) -> (F, F, F, F, F) {
    let t6174 = t6173 * t954;
    let t6177 = t6157 * t2970;
    let t6184 = t2974 + 0.61805555555555555556e-2 * t4571 - 0.61805555555555555555e-2 * t6094 + 0.18541666666666666667e-1 * t6098 - 0.92708333333333333333e-2 * t6102;
    let t6185 = t6184 * t324;
    let t6189 = t1633 * t1633;
    (t6174, t6177, t6184, t6185, t6189)
}
