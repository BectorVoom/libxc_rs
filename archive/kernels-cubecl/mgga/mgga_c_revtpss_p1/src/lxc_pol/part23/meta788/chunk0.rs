//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2601/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2601<F: Float>(t14472: F, t1580: F, t2439: F, t2444: F, t6049: F, t689: F, t136: F, t2457: F, t41011: F, t6048: F, t10504: F, t6071: F) -> (F, F, F, F) {
    let t61400 = t2439 * t14472 * t1580;
    let t61403 = t689 * t2444 * t6049;
    let t61407 = t41011 * t6048 * t136 * t2457;
    let t61411 = t10504 * t6071 * t136 * t2457;
    (t61400, t61403, t61407, t61411)
}
