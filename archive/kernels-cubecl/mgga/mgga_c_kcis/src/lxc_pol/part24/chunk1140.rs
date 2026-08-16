//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1140/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1140<F: Float>(t2157: F, t36543: F, t695: F, t2728: F, t2740: F, t8786: F, t887: F, t26463: F, t2746: F, t882: F, t888: F, t91909: F, t91913: F, t91916: F, t91919: F, t91922: F, t91925: F, t91929: F, t91932: F, t91935: F, t91938: F, t91941: F, t91944: F, t91948: F) -> (F, F) {
    let t91951 = t36543 * t695 * t2157;
    let t91955 = t8786 * t2728 * t2740 * t887;
    let t91956 = t26463 * t91955;
    let t91961 = t26463 * t8786 * t888 * t882 * t2746;
    let t91963 = -F::cast_from(0.55715333333333333331e-1_f64) * t91909 - F::cast_from(0.41786499999999999999e-1_f64) * t91913 - F::cast_from(0.65001222222222222219e-1_f64) * t91916 - F::cast_from(0.27857666666666666666e-1_f64) * t91919 + F::cast_from(0.27857666666666666666e-1_f64) * t91922 + F::cast_from(0.69644166666666666665e-2_f64) * t91925 - F::cast_from(0.69644166666666666665e-2_f64) * t91929 + F::cast_from(0.55715333333333333331e-1_f64) * t91932 + F::cast_from(0.41786499999999999999e-1_f64) * t91935 - F::cast_from(0.2089325e-1_f64) * t91938 - F::cast_from(0.2089325e-1_f64) * t91941 + F::cast_from(0.65001222222222222219e-1_f64) * t91944 + F::cast_from(0.41786499999999999999e-1_f64) * t91948 - F::cast_from(0.69505208333333333333e-3_f64) * t91951 + F::cast_from(0.41703125000000000001e-2_f64) * t91956 - F::cast_from(0.41703125000000000001e-2_f64) * t91961;
    (t91955, t91963)
}
