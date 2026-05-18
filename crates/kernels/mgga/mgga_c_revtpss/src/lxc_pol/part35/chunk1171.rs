//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1171/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1171<F: Float>(t110275: F, t93317: F, t30400: F, t689: F, t25431: F, t25411: F, t105946: F, t7407: F, t106387: F, t30356: F, t686: F, t72: F) -> (F, F, F, F, F, F) {
    let t110276 = t93317 * t110275;
    let t110288 = t30400 * t689;
    let t110289 = t25431 * t110288;
    let t110291 = t25411 * t110288;
    let t110316 = t105946 * t7407;
    let t110318 = t106387 * t7407;
    let t110322 = t30356 * t72 * t686;
    (t110276, t110289, t110291, t110316, t110318, t110322)
}
