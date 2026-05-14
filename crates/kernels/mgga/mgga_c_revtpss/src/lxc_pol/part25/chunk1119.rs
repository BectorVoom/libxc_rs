//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1119/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1119<F: Float>(t25411: F, t93123: F, t1955: F, t92888: F, t231: F, t2828: F, t836: F, t7056: F, t9646: F, t1949: F, t22: F, t25402: F, t1954: F, t39643: F, t25296: F, t25310: F) -> (F, F, F, F, F, F, F) {
    let t93124 = t25411 * t93123;
    let t93126 = t1955 * t92888;
    let t93130 = t2828 * t836 * t231;
    let t93134 = t9646 * t7056;
    let t93136 = t25402 * t1949 * t22;
    let t93138 = 0.43639970290213137151e-3 * t93134 * t93136;
    let t93139 = t1954 * t39643;
    let t93140 = t93139 * t7056;
    let t93142 = 0.51727911450665971904e-3 * t93140 * t93136;
    let t93143 = t25310 * t25296;
    (t93124, t93126, t93130, t93138, t93139, t93142, t93143)
}
