//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 628/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk628<F: Float>(t3622: F, t6737: F, t3644: F, t3658: F, t4787: F, t5017: F, t5023: F, t6484: F, t6489: F, t6494: F, t6499: F, t6502: F, t6506: F, t6510: F) -> (F, F) {
    let t6738 = t6737 * t3622;
    let t6751 = F::cast_from(0.890445125e-2_f64) * t3644 * t6738 + F::cast_from(0.17411041666666666666e-2_f64) * t6484 + F::cast_from(0.34822083333333333332e-2_f64) * t6489 - F::cast_from(0.23214722222222222222e-2_f64) * t6494 - F::cast_from(0.38691203703703703703e-3_f64) * t6499 + F::cast_from(0.23214722222222222222e-2_f64) * t6502 + F::cast_from(0.11607361111111111111e-2_f64) * t6506 + F::cast_from(0.19345601851851851852e-2_f64) * t6510 - t3658 - F::cast_from(0.23214722222222222222e-2_f64) * t5017 + F::cast_from(0.15476481481481481481e-2_f64) * t5023 + F::cast_from(0.23214722222222222222e-2_f64) * t4787;
    (t6738, t6751)
}
