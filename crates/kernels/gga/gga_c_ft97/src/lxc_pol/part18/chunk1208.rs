//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1208/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1208<F: Float>(t1808: F, t22952: F, t22953: F, t5691: F, t920: F, t3051: F, t5664: F, t18: F, t473: F, t1767: F, t25893: F, t1308: F, t3000: F, t358: F, t89: F, t1637: F, t6516: F) -> (F, F, F, F, F) {
    let t101858 = t22952 * t22953 * t5691 * t920 * t1808;
    let t101860 = t5664 * t3051;
    let t101864 = t101860 * t22953 * t5691 * t18 * t473;
    let t101869 = t25893 * t22953 * t5691 * t920 * t1767;
    let t101873 = t89 * t3000 * t1308 * t358;
    let t101876 = t89 * t1637 * t6516;
    (t101858, t101864, t101869, t101873, t101876)
}
