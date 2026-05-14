//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1149/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1149<F: Float>(t100252: F, t11772: F, t1307: F, t28: F, t469: F, t5665: F, t25846: F, t432: F, t1317: F, t1800: F, t100214: F, t100219: F, t100224: F, t100228: F, t100232: F, t100236: F, t100241: F, t100246: F, t100250: F) -> (F, F, F, F) {
    let t100253 = 4.0 / 27.0 * t100252;
    let t100257 = t5665 * t28 * t469 * t1307 * t11772;
    let t100259 = t25846 * t432;
    let t100262 = t1317 * t28 * t1800 * t100259;
    let t100264 = t100214 / 4.0 - t100219 / 3.0 + t100224 / 2.0 - 2.0 / 9.0 * t100228 - 4.0 / 9.0 * t100232 + 2.0 / 27.0 * t100236 + t100241 / 6.0 + t100246 / 3.0 + 10.0 / 81.0 * t100250 + t100253 + t100257 / 12.0 + 2.0 / 3.0 * t100262;
    (t100257, t100259, t100262, t100264)
}
