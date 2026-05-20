//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1720/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1720<F: Float>(t1285: F, t12865: F, t372: F, t5302: F, t4181: F, t5405: F, t13396: F, t1042: F, t3588: F, t3603: F, t5332: F, t3720: F) -> (F, F, F, F, F) {
    let t17693 = t1285 * t12865;
    let t17694 = t372 * t5302;
    let t17695 = t4181 * t5405;
    let t17696 = t17694 * t17695;
    let t17699 = t5302 * t13396;
    let t17700 = t1042 * t17699;
    let t17703 = t3603 * t3588;
    let t17704 = t5332 * t17703;
    let t17705 = t3720 * t17704;
    (t17693, t17695, t17696, t17700, t17705)
}
