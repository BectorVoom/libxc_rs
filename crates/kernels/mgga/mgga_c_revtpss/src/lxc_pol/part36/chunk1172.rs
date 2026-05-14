//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1172/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1172<F: Float>(t113097: F, t113100: F, t113104: F, t113108: F, t113111: F, t113115: F, t113123: F, t113416: F, t113420: F, t113424: F, t113428: F, t113433: F, t1940: F, t1963: F, t1964: F, t2403: F, t25206: F, t27158: F, t27368: F, t27382: F, t29599: F, t29602: F, t29716: F, t30: F, t7091: F, t7783: F, t98637: F) -> (F,) {
    let t113439 = -9.0 * t27158 * t113097 + 9.0 * t27158 * t113100 - 9.0 / 2.0 * t25206 * t113104 + 3.0 * t27382 * t113108 - 3.0 / 2.0 * t1940 * t7091 * t113111 - 9.0 * t25206 * t113115 - 9.0 * t98637 * t29599 - 3.0 * t1940 * t27368 * t29716 + 3.0 * t113123 * t1964 + t1940 * t113416 * t30 / 2.0 + 9.0 / 2.0 * t2403 * t1963 * t113420 - t1940 * t7091 * t113424 / 2.0 + 3.0 / 2.0 * t2403 * t1963 * t113428 - 9.0 / 2.0 * t25206 * t113433 + 9.0 * t2403 * t7783 * t29602;
    (t113439,)
}
