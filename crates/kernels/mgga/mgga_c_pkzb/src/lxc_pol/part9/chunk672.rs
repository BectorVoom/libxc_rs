//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 672/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk672<F: Float>(t3186: F, t3187: F, t406: F, t1229: F, t2099: F, t918: F, t1249: F, t2382: F, t2381: F, t3032: F, t3035: F, t3037: F, t3040: F, t3072: F, t3076: F, t3144: F, t3146: F, t3149: F, t3151: F, t3155: F, t3159: F, t3164: F) -> (F, F, F, F, F, F) {
    let t3188 = t3186 * t3187;
    let t3189 = t406 * t3188;
    let t3192 = t2099 * t1229;
    let t3193 = t918 * t3192;
    let t3195 = t1249 * t2382;
    let t3196 = t2381 * t3195;
    let t3199 = -t3032 + t3035 + t3037 - t3040 + t3072 + t3076 + t3144 + t3146 - t3149 - t3151 + t3155 - t3159 - t3164;
    (t3188, t3189, t3193, t3195, t3196, t3199)
}
