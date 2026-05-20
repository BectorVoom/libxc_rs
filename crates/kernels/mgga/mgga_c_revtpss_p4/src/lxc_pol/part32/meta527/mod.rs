//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta527 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1832;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta527<F: Float>(t2482: F, t596: F, t7043: F, t2677: F, t240: F, t25260: F, t25228: F, t9775: F, t233: F, t41077: F, t7056: F, t9646: F) -> (F, F, F, F, F, F) {
        let (t93072, t93073, t93082, t93088, t93118, t93134) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1832::<F>(t2482, t596, t7043, t2677, t240, t25260, t25228, t9775, t233, t41077, t7056, t9646);
    (t93072, t93073, t93082, t93088, t93118, t93134)
}
