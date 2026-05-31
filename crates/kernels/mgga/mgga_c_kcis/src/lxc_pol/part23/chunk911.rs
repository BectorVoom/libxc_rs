//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 911/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk911<F: Float>(t1380: F, t16984: F, t613: F, t1368: F, t16830: F, t16940: F, t16944: F, t16946: F, t16951: F, t16954: F, t16959: F, t16965: F, t16970: F, t16976: F, t16981: F, t1930: F, t3981: F, t4009: F, t493: F, t5691: F) -> F {
    let t16985 = t16984 * t1380;
    let t16986 = t613 * t16985;
    let t16989 = -t16940 + t16944 - t1368 * t16946 / F::cast_from(288.0_f64) - t1368 * t16951 / F::cast_from(216.0_f64) + t16954 / F::cast_from(162.0_f64) - t5691 * t3981 / F::cast_from(81.0_f64) + t1368 * t16959 / F::cast_from(48.0_f64) - t16830 * t16965 / F::cast_from(108.0_f64) + t16830 * t16970 / F::cast_from(72.0_f64) - t493 * t16976 / F::cast_from(96.0_f64) - t16981 + t1930 * t4009 / F::cast_from(36.0_f64) + t1368 * t16986 / F::cast_from(24.0_f64);
    t16989
}
