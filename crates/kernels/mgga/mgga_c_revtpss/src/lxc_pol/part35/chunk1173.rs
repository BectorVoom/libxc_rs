//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1173/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1173<F: Float>(t30384: F, t786: F, t789: F, t30395: F, t689: F, t25431: F, t25411: F, t6072: F, t7384: F, t30341: F, t686: F, t72: F) -> (F, F, F, F, F) {
    let t110459 = t786 * t30384 * t789;
    let t110475 = t30395 * t689;
    let t110476 = t25431 * t110475;
    let t110478 = t25411 * t110475;
    let t110489 = t689 * t7384 * t6072;
    let t110502 = t30341 * t72 * t686;
    (t110459, t110476, t110478, t110489, t110502)
}
