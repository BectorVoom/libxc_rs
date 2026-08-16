//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1408/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1408<F: Float>(t1284: F, t5216: F, t1204: F, t5477: F, t17814: F, t3783: F, t3302: F, t3588: F, t471: F, t5332: F, t1269: F, t3781: F) -> (F, F, F, F, F) {
    let t17861 = t5216 * t1284;
    let t17864 = t1204 * t5477;
    let t17869 = t17814 * t3783;
    let t17875 = t3302 * t3588 * t471;
    let t17876 = t5332 * t17875;
    let t17879 = t3781 * t1269;
    (t17861, t17864, t17869, t17876, t17879)
}
