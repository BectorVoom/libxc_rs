//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 829/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk829(t5379: f64, t5393: f64, t1282: f64, t1291: f64, t187: f64, t1872: f64, t3664: f64, t3669: f64, t437: f64, t5035: f64, t5037: f64, t5038: f64, t5041: f64, t5190: f64, t5358: f64, t5360: f64, t5363: f64) -> (f64, f64) {
    let t5394 = t5379 + t5393;
    let t5398 = t5035 - t5037 - t5038 + t5041 - t5190 + t187 * (-t1282 * t5394 - t1291 * t5360 - t1872 * t3664 + 2.0_f64 * t3669 * t5363 + t437 * t5358 - t5035 + t5037 + t5038 - t5041 + t5190);
    (t5394, t5398)
}
