//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1264/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1264<F: Float>(t1434: F, t31015: F, t681: F, t109335: F, t17785: F, t1901: F, t27787: F, t27819: F, t27820: F, t729: F, t2: F, t30859: F, t2354: F, t6118: F, t684: F, t124225: F, t124229: F, t124232: F, t124235: F, t124240: F, t124244: F, t124247: F, t97123: F) -> (F, F, F, F, F) {
    let t124250 = t1434 * t681 * t31015;
    let t124253 = t1901 * t109335 * t17785;
    let t124257 = t27819 * t729 * t27787 * t27820;
    let t124259 = t2 * t30859;
    let t124262 = t6118 * t2354 * t124259 * t684;
    let t124264 = 4.0 / 27.0 * t97123 - t124225 / 18.0 + t124229 / 9.0 - t124232 / 9.0 + t124235 / 24.0 + t124240 / 12.0 + t124244 / 3.0 + 4.0 / 9.0 * t124247 - 2.0 / 9.0 * t124250 + 4.0 / 3.0 * t124253 - t124257 / 4.0 + t124262 / 18.0;
    (t124250, t124253, t124257, t124262, t124264)
}
