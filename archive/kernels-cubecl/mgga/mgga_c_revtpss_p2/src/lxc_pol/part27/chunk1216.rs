//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1216/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1216<F: Float>(t233: F, t41077: F, t25348: F, t689: F, t25411: F, t1955: F, t92888: F, t231: F, t2828: F, t836: F, t7056: F, t9646: F) -> (F, F, F, F, F, F) {
    let t93118 = t41077 * t233;
    let t93123 = t25348 * t689;
    let t93124 = t25411 * t93123;
    let t93126 = t1955 * t92888;
    let t93130 = t2828 * t836 * t231;
    let t93134 = t9646 * t7056;
    (t93118, t93123, t93124, t93126, t93130, t93134)
}
