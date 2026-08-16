//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1499/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1499<F: Float>(t4321: F, t6072: F, t689: F, t23383: F, t2465: F, t686: F, t72: F, t10995: F, t23403: F, t212: F, t23359: F, t780: F) -> (F, F, F, F) {
    let t76051 = t689 * t4321 * t6072;
    let t76058 = t2465 * t23383 * t72 * t686;
    let t76062 = t10995 * t23403 * t72 * t686;
    let t76081 = t689 * t212 * t23359 * t780;
    (t76051, t76058, t76062, t76081)
}
