//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1882/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1882<F: Float>(t14701: F, t92955: F, t241: F, t820: F, t93060: F, t4447: F, t92951: F, t14727: F, t25227: F, t2661: F, t4430: F, t93034: F) -> (F, F, F, F, F) {
    let t98983 = t92955 * t14701;
    let t98988 = t820 * t93060 * t241;
    let t98991 = t92951 * t4447;
    let t99000 = t2661 * t25227 * t14727;
    let t99002 = t93034 * t4430;
    (t98983, t98988, t98991, t99000, t99002)
}
