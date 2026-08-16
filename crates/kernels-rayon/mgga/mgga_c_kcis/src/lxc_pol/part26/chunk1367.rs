//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1367/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1367(t29393: f64, t7904: f64, t102357: f64, t103303: f64, t103394: f64, t28369: f64, t28388: f64, t28551: f64, t7908: f64, t94472: f64, t94489: f64, t94492: f64, t98489: f64, t98491: f64, t98519: f64) -> f64 {
    let t103467 = t29393 * t7904;
    let t103475 = 0.27802083333333333334e-2_f64 * t7908 * t103394 + 0.27802083333333333334e-2_f64 * t7908 * t103303 + 0.37134344353515625001e-4_f64 * t28388 * t103303 - 0.16581944444444444444e-2_f64 * t102357 - 0.23168402777777777778e-3_f64 * t103467 + 0.46336805555555555556e-3_f64 * t28369 * t28551 + t98489 - 0.20594135802469135803e-3_f64 * t98491 - 0.36848765432098765431e-3_f64 * t94472 - 0.15445601851851851852e-3_f64 * t94489 - 0.15445601851851851852e-3_f64 * t94492 - t98519;
    t103475
}
