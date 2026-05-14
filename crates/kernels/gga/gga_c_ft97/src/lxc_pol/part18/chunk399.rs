//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 399/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk399<F: Float>(t18: F, t464: F, t463: F, t458: F, t963: F, t1787: F, t3009: F, t2: F, t942: F, t1587: F, t432: F, t24: F, t3103: F, t469: F, t1773: F, t1776: F, t1778: F, t3125: F, t3128: F, t3131: F, t3135: F, t3139: F, t462: F, t92: F) -> (F, F, F, F, F, F, F) {
    let t3140 = t464 * t18;
    let t3141 = t463 * t3140;
    let t3144 = t458 * t963;
    let t3146 = t1787 * t3009;
    let t3149 = t2 * t942;
    let t3151 = t1587 * t3149 * t432;
    let t3155 = t24 * t469 * t3103;
    let t3157 = t1773 + t1776 / 9.0 + t1778 / 3.0 + t3125 / 9.0 - 2.0 / 9.0 * t462 * t3128 + t462 * t3131 / 3.0 + 2.0 / 3.0 * t462 * t3135 - 2.0 / 3.0 * t3139 * t3141 + t3144 / 3.0 + t462 * t3146 / 3.0 + 2.0 * t462 * t3151 - t92 * t3155;
    (t3140, t3141, t3146, t3149, t3151, t3155, t3157)
}
