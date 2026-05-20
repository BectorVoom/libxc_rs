//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta572 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1987;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1988;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta572<F: Float>(t2482: F, t596: F, t7043: F, t2677: F, t240: F, t25260: F, t25228: F, t9775: F, t10073: F, t25308: F, t25403: F, t25402: F, t7048: F, t7056: F, t233: F, t41077: F, t9646: F, t1949: F, t22: F, t1954: F, t39643: F, t2470: F, t25295: F, t7058: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t93072, t93073, t93082, t93088, t93112, t93116) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1987::<F>(t2482, t596, t7043, t2677, t240, t25260, t25228, t9775, t10073, t25308, t25403, t25402, t7048, t7056);
        let (t93118, t93138, t93139, t93142, t93150, t93151) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1988::<F>(t233, t41077, t7056, t9646, t1949, t22, t25402, t1954, t39643, t2470, t25295, t7058);
    (t93072, t93073, t93082, t93088, t93112, t93116, t93118, t93138, t93139, t93142, t93150, t93151)
}
