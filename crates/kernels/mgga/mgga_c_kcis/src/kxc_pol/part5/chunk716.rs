//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 716/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk716<F: Float>(t5379: F, t5393: F, t1282: F, t1291: F, t187: F, t1872: F, t3664: F, t3669: F, t437: F, t5035: F, t5037: F, t5038: F, t5041: F, t5190: F, t5358: F, t5360: F, t5363: F) -> (F, F) {
    let t5394 = t5379 + t5393;
    let t5398 = t5035 - t5037 - t5038 + t5041 - t5190 + t187 * (-t1282 * t5394 - t1291 * t5360 - t1872 * t3664 + F::cast_from(2.0_f64) * t3669 * t5363 + t437 * t5358 - t5035 + t5037 + t5038 - t5041 + t5190);
    (t5394, t5398)
}
