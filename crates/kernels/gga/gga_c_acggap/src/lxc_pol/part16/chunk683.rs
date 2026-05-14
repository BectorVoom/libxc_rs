//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 683/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk683<F: Float>(t2133: F, t322: F, t2132: F, t7896: F, t1960: F, t872: F, t2139: F, t463: F, t2147: F, t2138: F, t441: F, t618: F) -> (F, F, F, F, F, F) {
    let t7897 = t2133 * t322;
    let t7898 = t2132 * t7897;
    let t7900 = 0.34694512752820797848e1 * t7896 * t7898;
    let t7901 = t1960 * t872;
    let t7907 = t2139 * t463;
    let t7908 = t2147 * t7907;
    let t7909 = t2138 * t7908;
    let t7911 = t441 * t618;
    (t7898, t7900, t7901, t7908, t7909, t7911)
}
