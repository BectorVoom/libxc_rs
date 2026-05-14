//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1130/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1130<F: Float>(t108978: F, t1926: F, t1927: F, t5872: F, t2247: F, t5826: F, t60673: F, t6957: F, t30110: F, t531: F, t1913: F, t7956: F, t30197: F, t571: F, t2045: F, t6936: F) -> (F, F, F, F, F, F, F, F) {
    let t108979 = t1926 * t108978;
    let t108986 = t1927 * t5872;
    let t108987 = t1926 * t108986;
    let t108990 = t2247 * t5826;
    let t108995 = t60673 * t6957;
    let t109173 = t531 * t30110;
    let t109339 = t1913 * t7956;
    let t109345 = t571 * t30197;
    let t109348 = t6936 * t2045;
    (t108979, t108987, t108990, t108995, t109173, t109339, t109345, t109348)
}
