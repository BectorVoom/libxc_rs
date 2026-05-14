//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1310/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1310<F: Float>(t7272: F, t7483: F, t20911: F, t7275: F, t7282: F, t7411: F, t20896: F, t7286: F, t1987: F, t9245: F, t2848: F, t721: F, t21267: F, t7308: F, t20893: F, t2751: F) -> (F, F, F, F, F, F, F, F) {
    let t25883 = 4.0 * t7483 * t7272;
    let t25885 = 0.19298375398431042081e3 * t20911 * t7275;
    let t25887 = 0.32163958997385070134e2 * t7411 * t7282;
    let t25889 = 0.1034520258385468006e4 * t20896 * t7286;
    let t25891 = 0.70178683471615754484e1 * t1987 * t9245;
    let t25892 = t721 * t2848;
    let t25895 = 0.41016075432865626631e4 * t21267 * t7308 * t25892;
    let t25897 = 8.0 * t20893 * t2751;
    (t25883, t25885, t25887, t25889, t25891, t25892, t25895, t25897)
}
