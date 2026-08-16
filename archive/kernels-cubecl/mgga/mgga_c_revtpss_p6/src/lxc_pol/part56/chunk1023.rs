//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1023/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1023<F: Float>(t26948: F, t487: F, t1269: F, t7642: F, t3596: F, t37885: F, t13181: F, t3140: F, t1892: F, t7063: F, t30: F, t41154: F) -> (F, F, F, F, F, F) {
    let t97040 = t26948 * t487;
    let t97081 = t7642 * t1269;
    let t97312 = t37885 * t3596;
    let t97346 = t3140 * t13181;
    let t98040 = t7063 * t1892;
    let t98785 = t41154 * t30;
    (t97040, t97081, t97312, t97346, t98040, t98785)
}
