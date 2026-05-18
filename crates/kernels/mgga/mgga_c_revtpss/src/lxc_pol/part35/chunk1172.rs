//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1172/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1172<F: Float>(t110322: F, t25387: F, t30380: F, t686: F, t72: F, t7058: F, t28314: F, t99466: F, t7064: F, t103067: F, t4481: F, t27216: F, t28360: F) -> (F, F, F, F, F, F) {
    let t110323 = t25387 * t110322;
    let t110339 = t30380 * t72 * t686;
    let t110340 = t7058 * t110339;
    let t110344 = t99466 * t28314;
    let t110346 = t7064 * t110339;
    let t110355 = t103067 * t4481;
    let t110453 = t27216 * t28360;
    (t110323, t110340, t110344, t110346, t110355, t110453)
}
