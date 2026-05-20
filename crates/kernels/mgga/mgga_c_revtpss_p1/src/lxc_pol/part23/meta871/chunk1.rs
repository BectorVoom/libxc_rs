//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2771/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2771<F: Float>(t22061: F, t808: F, t9845: F, t22085: F, t9962: F, t22182: F, t47215: F, t22021: F, t9793: F, t9794: F, t6876: F, t9909: F) -> (F, F, F, F, F) {
    let t74304 = t9845 * t808 * t22061;
    let t74319 = t9962 * t22085;
    let t74322 = t47215 * t22182;
    let t74341 = t9793 * t9794 * t22021;
    let t74358 = t9909 * t6876;
    (t74304, t74319, t74322, t74341, t74358)
}
