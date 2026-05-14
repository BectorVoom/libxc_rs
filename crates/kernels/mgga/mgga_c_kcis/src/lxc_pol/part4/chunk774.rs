//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 774/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk774<F: Float>(t5084: F, t5087: F, t5089: F, t5092: F, t5094: F, t5097: F, t5100: F, t5166: F, t5170: F, t5173: F, t5178: F, t5183: F, t5186: F, t5379: F, t1282: F, t1291: F, t187: F, t1872: F, t3664: F, t3669: F, t437: F, t5035: F, t5037: F, t5038: F, t5041: F, t5190: F, t5358: F, t5360: F, t5363: F) -> (F, F) {
    let t5393 = -0.9375e-1 * t5084 + 0.71944444444444444443e-1 * t5087 + 0.101171875e-1 * t5089 - 0.625e-1 * t5092 + 0.53958333333333333333e-1 * t5094 - 0.53958333333333333333e-1 * t5097 + 0.13489583333333333333e-1 * t5100 + 0.9375e-1 * t5166 - 0.101171875e-1 * t5170 + 0.101171875e-1 * t5173 - 0.20234375e-1 * t5178 - 0.44965277777777777777e-2 * t5183 - 0.16666666666666666667e0 * t5186;
    let t5394 = t5379 + t5393;
    let t5398 = t5035 - t5037 - t5038 + t5041 - t5190 + t187 * (-t1282 * t5394 - t1291 * t5360 - t1872 * t3664 + 2.0 * t3669 * t5363 + t437 * t5358 - t5035 + t5037 + t5038 - t5041 + t5190);
    (t5394, t5398)
}
