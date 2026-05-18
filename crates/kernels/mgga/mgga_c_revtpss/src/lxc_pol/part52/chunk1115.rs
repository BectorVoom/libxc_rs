//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1115/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1115<F: Float>(t670: F, t7968: F, t8107: F, t9593: F, t7983: F, t84: F, t8440: F, t25081: F, t8567: F, t31844: F, t8478: F, t8479: F) -> (F, F, F, F, F, F) {
    let t111018 = t7968 * t670;
    let t111176 = t8107 * t9593;
    let t111371 = t670 * t7983;
    let t119457 = t8440 * t84;
    let t119578 = t8567 * t25081;
    let t119751 = t8478 * t8479 * t31844;
    (t111018, t111176, t111371, t119457, t119578, t119751)
}
