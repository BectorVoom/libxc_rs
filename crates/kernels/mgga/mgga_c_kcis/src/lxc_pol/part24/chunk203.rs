//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 203/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk203<F: Float>(t887: F, t888: F, t221: F, t712: F, t750: F, t777: F, t870: F, t874: F, t227: F) -> (F, F, F, F) {
    let t889 = t887 * t888;
    let t895 = t870 * t221 - F::cast_from(0.66725e-1_f64) * t874 * t889 - F::cast_from(0.92858888888888888886e-2_f64) * t712 + F::cast_from(0.69644166666666666665e-2_f64) * t750 - F::cast_from(0.69644166666666666665e-2_f64) * t777;
    let t897 = t227 * t227;
    let t898 = F::cast_from(1.0_f64) / t897;
    (t889, t895, t897, t898)
}
