//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1421/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1421<F: Float>(t15014: F, t9303: F, t10982: F, t1568: F, t9646: F, t14986: F, t2453: F, t14567: F, t14557: F, t4519: F, t9292: F, t2798: F, t4499: F, t9288: F) -> (F, F, F, F, F, F, F) {
    let t51237 = t9303 * t15014;
    let t51246 = t9646 * t1568 * t10982;
    let t51258 = t2453 * t14986;
    let t51297 = t2453 * t14567;
    let t51390 = t9303 * t14557;
    let t51403 = t9292 * t4519;
    let t51408 = t2798 * t4499 * t9288;
    (t51237, t51246, t51258, t51297, t51390, t51403, t51408)
}
