//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2742/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2742<F: Float>(t12916: F, t20951: F, t5340: F, t17396: F, t17620: F, t17472: F, t5373: F, t1222: F, t17471: F, t20266: F, t17351: F, t20770: F, t56756: F) -> (F, F, F, F, F) {
    let t71845 = t5340 * t12916 * t20951;
    let t71859 = t17396 * t17620;
    let t71880 = t5373 * t17472;
    let t71883 = t1222 * t17471 * t20266;
    let t71886 = t17351 * t56756 * t20770;
    (t71845, t71859, t71880, t71883, t71886)
}
