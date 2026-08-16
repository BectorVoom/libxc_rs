//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 654/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk654(t3685: f64, t3698: f64, t1282: f64, t1291: f64, t187: f64, t3324: f64, t3327: f64, t3333: f64, t3482: f64, t3662: f64, t3664: f64, t3669: f64, t3670: f64, t437: f64) -> (f64, f64) {
    let t3699 = t3685 + t3698;
    let t3703 = t3324 - t3327 + t3333 - t3482 + t187 * (-t1282 * t3699 - 2.0_f64 * t1291 * t3664 + t3662 * t437 + 2.0_f64 * t3669 * t3670 - t3324 + t3327 - t3333 + t3482);
    (t3699, t3703)
}
