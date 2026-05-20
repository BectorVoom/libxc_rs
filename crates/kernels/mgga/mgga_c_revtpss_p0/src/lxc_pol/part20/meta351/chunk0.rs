//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1279/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1279<F: Float>(t4144: F, t9593: F, t159: F, t2698: F, t4135: F, t4147: F, t26: F, t65: F, t9163: F, t99: F, t107: F, t9232: F) -> (F, F, F, F, F, F) {
    let t25177 = t9593 * t4144;
    let t25273 = t2698 * t159;
    let t25802 = t4147 * t4135;
    let t33127 = F::new(1.0) / t65 / t26;
    let t36227 = t99 * t9163;
    let t36415 = t107 * t9232;
    (t25177, t25273, t25802, t33127, t36227, t36415)
}
