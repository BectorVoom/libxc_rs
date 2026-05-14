//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 631/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk631<F: Float>(t6006: F, t6317: F, t6316: F, t4229: F, t491: F, t4304: F, t79: F, t1493: F, t2259: F, t4231: F, t5996: F, t4230: F, t4208: F, t469: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t6318 = t6317 * t6006;
    let t6319 = t6316 * t6318;
    let t6321 = t491 * t4229;
    let t6322 = t79 * t4304;
    let t6323 = t6322 * t6006;
    let t6324 = t6321 * t6323;
    let t6326 = t2259 * t1493;
    let t6328 = t4231 * t5996;
    let t6329 = t4230 * t6328;
    let t6331 = t4208 * t469;
    (t6318, t6319, t6321, t6322, t6323, t6324, t6326, t6328, t6329, t6331)
}
