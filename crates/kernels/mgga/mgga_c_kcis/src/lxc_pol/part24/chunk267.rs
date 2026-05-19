//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 267/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk267<F: Float>(t1045: F, t1114: F, t345: F, t1100: F, t1102: F, t1106: F, t1111: F, t278: F, t344: F, t975: F) -> (F, F, F) {
    let t1115 = t1114 * t1045;
    let t1116 = t345 * t1115;
    let t1121 = t1100 + F::cast_from(0.65704296666666666667e-3_f64) * t1102 * t1106 + F::cast_from(0.1478346675e-2_f64) * t344 * t1111 - F::new(0.98556445e-3) * t344 * t1116 - F::new(4.0) * t278 * t975;
    (t1115, t1116, t1121)
}
