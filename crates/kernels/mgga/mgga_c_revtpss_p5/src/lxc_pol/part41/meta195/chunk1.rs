//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 789/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk789<F: Float>(t1263: F, t1774: F, t1122: F, t1042: F, t5062: F, t5065: F, t5067: F, t5070: F, t5107: F, t5111: F, t5189: F, t5191: F, t5194: F, t5196: F, t5200: F, t5204: F, t5209: F) -> (F, F, F, F) {
    let t5277 = t1263 * t1774;
    let t5278 = t5277 * t1122;
    let t5279 = t1042 * t5278;
    let t5284 = -t5062 + t5065 + t5067 - t5070 + t5107 + t5111 + t5189 + t5191 - t5194 - t5196 + t5200 - t5204 - t5209;
    (t5277, t5278, t5279, t5284)
}
