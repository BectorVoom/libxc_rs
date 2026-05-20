//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta297 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1171;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta297<F: Float>(t1145: F, t12277: F, t141: F, t3362: F, t606: F, t2258: F, t3417: F, t3367: F, t3360: F, t128: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t12278, t12279, t12281, t12282, t12283, t12284, t12286, t12287, t12288, t12289, t12291, t12292) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1171::<F>(t1145, t12277, t141, t3362, t606, t2258, t3417, t3367, t3360, t128);
    (t12278, t12279, t12281, t12282, t12283, t12284, t12286, t12287, t12288, t12289, t12291, t12292)
}
