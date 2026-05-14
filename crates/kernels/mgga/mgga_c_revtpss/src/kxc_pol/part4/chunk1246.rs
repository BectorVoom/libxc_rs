//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1246/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1246<F: Float>(t4181: F, t5405: F, t17694: F, t13396: F, t5302: F, t1042: F, t3588: F, t3603: F, t5332: F, t3720: F, t15904: F, t3623: F, t13148: F, t11249: F, t1794: F) -> (F, F, F, F, F, F, F) {
    let t17695 = t4181 * t5405;
    let t17696 = t17694 * t17695;
    let t17699 = t5302 * t13396;
    let t17700 = t1042 * t17699;
    let t17703 = t3603 * t3588;
    let t17704 = t5332 * t17703;
    let t17705 = t3720 * t17704;
    let t17708 = t3623 * t15904;
    let t17709 = t13148 * t17708;
    let t17710 = t1794 * t11249;
    (t17695, t17696, t17700, t17705, t17708, t17709, t17710)
}
