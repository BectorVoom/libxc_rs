//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2757/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2757<F: Float>(t22449: F, t2435: F, t136: F, t2457: F, t6918: F, t9674: F, t13999: F, t22146: F, t22145: F, t48863: F, t49137: F, t124: F, t6861: F) -> (F, F, F, F, F) {
    let t73707 = t2435 * t22449;
    let t73712 = t9674 * t6918 * t136 * t2457;
    let t73726 = t13999 * t22146;
    let t73729 = t49137 * t48863 * t22145;
    let t73731 = t124 * t6861;
    (t73707, t73712, t73726, t73729, t73731)
}
