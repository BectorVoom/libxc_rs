//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1178/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1178<F: Float>(t240: F, t25981: F, t2661: F, t9935: F, t25987: F, t9775: F, t25986: F, t9769: F, t25978: F, t4014: F, t25972: F, t9923: F, t2453: F, t4086: F, t64: F, t9795: F) -> (F, F, F, F, F, F) {
    let t94550 = t25981 * t240;
    let t94552 = t2661 * t94550 * t9935;
    let t94554 = t9775 * t25987;
    let t94557 = t2661 * t25986 * t9769;
    let t94559 = t25978 * t4014;
    let t94561 = t25972 * t9923;
    let t94564 = t2453 * t4086 * t64;
    let t94565 = t94564 * t9795;
    (t94552, t94554, t94557, t94559, t94561, t94565)
}
