//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1291/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1291<F: Float>(t102081: F, t102085: F, t102088: F, t102092: F, t27567: F, t27583: F, t29575: F, t94928: F, t94974: F, t94977: F, t99176: F, t99193: F, t99229: F, t99238: F) -> F {
    let t102098 = -F::cast_from(0.15445601851851851852e-3_f64) * t99176 + t99193 - F::cast_from(0.46336805555555555556e-3_f64) * t27583 * t102081 + F::cast_from(0.25794135802469135802e-2_f64) * t102085 + F::cast_from(0.15459116753472222222e-4_f64) * t27567 * t102088 + F::cast_from(0.11584201388888888889e-3_f64) * t102092 + t99229 + F::cast_from(0.23168402777777777778e-3_f64) * t94928 * t29575 + t99238 - F::cast_from(0.7722800925925925926e-4_f64) * t94974 - F::cast_from(0.7722800925925925926e-4_f64) * t94977;
    t102098
}
