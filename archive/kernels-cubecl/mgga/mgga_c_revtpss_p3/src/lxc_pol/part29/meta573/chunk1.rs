//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1921/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1921<F: Float>(t10744: F, t4353: F, t7028: F, t14701: F, t92955: F, t14707: F, t25270: F, t241: F, t820: F, t93060: F, t14896: F, t4447: F, t92951: F) -> (F, F, F, F, F) {
    let t98979 = t10744 * t7028 * t4353;
    let t98983 = t92955 * t14701;
    let t98985 = t25270 * t14707;
    let t98988 = t820 * t93060 * t241;
    let t98989 = t98988 * t14896;
    let t98991 = t92951 * t4447;
    (t98979, t98983, t98985, t98989, t98991)
}
