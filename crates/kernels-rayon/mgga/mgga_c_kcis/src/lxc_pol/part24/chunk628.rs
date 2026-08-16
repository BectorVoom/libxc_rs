//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 628/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk628(t3622: f64, t6737: f64, t3644: f64, t3658: f64, t4787: f64, t5017: f64, t5023: f64, t6484: f64, t6489: f64, t6494: f64, t6499: f64, t6502: f64, t6506: f64, t6510: f64) -> (f64, f64) {
    let t6738 = t6737 * t3622;
    let t6751 = 0.890445125e-2_f64 * t3644 * t6738 + 0.17411041666666666666e-2_f64 * t6484 + 0.34822083333333333332e-2_f64 * t6489 - 0.23214722222222222222e-2_f64 * t6494 - 0.38691203703703703703e-3_f64 * t6499 + 0.23214722222222222222e-2_f64 * t6502 + 0.11607361111111111111e-2_f64 * t6506 + 0.19345601851851851852e-2_f64 * t6510 - t3658 - 0.23214722222222222222e-2_f64 * t5017 + 0.15476481481481481481e-2_f64 * t5023 + 0.23214722222222222222e-2_f64 * t4787;
    (t6738, t6751)
}
