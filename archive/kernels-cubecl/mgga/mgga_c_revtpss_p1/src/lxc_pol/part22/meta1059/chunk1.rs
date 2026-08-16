//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3764/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3764<F: Float>(t1261: F, t20272: F, t247: F, t3634: F, t3584: F, t6573: F, t12916: F, t20951: F, t5340: F, t17170: F, t1774: F, t17396: F, t17620: F) -> (F, F, F, F, F) {
    let t71827 = t1261 * t247 * t3634 * t20272;
    let t71839 = t6573 * t3584;
    let t71845 = t5340 * t12916 * t20951;
    let t71854 = t1774 * t17170;
    let t71859 = t17396 * t17620;
    (t71827, t71839, t71845, t71854, t71859)
}
