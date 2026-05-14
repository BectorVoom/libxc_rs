//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1041/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1041<F: Float>(t30400: F, t689: F, t25431: F, t25411: F, t105946: F, t7407: F, t106387: F, t30356: F, t686: F, t72: F, t25387: F, t30380: F, t7058: F, t28314: F, t99466: F, t7064: F) -> (F, F, F, F, F, F, F, F, F) {
    let t110288 = t30400 * t689;
    let t110289 = t25431 * t110288;
    let t110291 = t25411 * t110288;
    let t110316 = t105946 * t7407;
    let t110318 = t106387 * t7407;
    let t110322 = t30356 * t72 * t686;
    let t110323 = t25387 * t110322;
    let t110339 = t30380 * t72 * t686;
    let t110340 = t7058 * t110339;
    let t110344 = t99466 * t28314;
    let t110346 = t7064 * t110339;
    (t110289, t110291, t110316, t110318, t110322, t110323, t110340, t110344, t110346)
}
