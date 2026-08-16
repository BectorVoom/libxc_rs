//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1140/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1140(t2157: f64, t36543: f64, t695: f64, t2728: f64, t2740: f64, t8786: f64, t887: f64, t26463: f64, t2746: f64, t882: f64, t888: f64, t91909: f64, t91913: f64, t91916: f64, t91919: f64, t91922: f64, t91925: f64, t91929: f64, t91932: f64, t91935: f64, t91938: f64, t91941: f64, t91944: f64, t91948: f64) -> (f64, f64) {
    let t91951 = t36543 * t695 * t2157;
    let t91955 = t8786 * t2728 * t2740 * t887;
    let t91956 = t26463 * t91955;
    let t91961 = t26463 * t8786 * t888 * t882 * t2746;
    let t91963 = -0.55715333333333333331e-1_f64 * t91909 - 0.41786499999999999999e-1_f64 * t91913 - 0.65001222222222222219e-1_f64 * t91916 - 0.27857666666666666666e-1_f64 * t91919 + 0.27857666666666666666e-1_f64 * t91922 + 0.69644166666666666665e-2_f64 * t91925 - 0.69644166666666666665e-2_f64 * t91929 + 0.55715333333333333331e-1_f64 * t91932 + 0.41786499999999999999e-1_f64 * t91935 - 0.2089325e-1_f64 * t91938 - 0.2089325e-1_f64 * t91941 + 0.65001222222222222219e-1_f64 * t91944 + 0.41786499999999999999e-1_f64 * t91948 - 0.69505208333333333333e-3_f64 * t91951 + 0.41703125000000000001e-2_f64 * t91956 - 0.41703125000000000001e-2_f64 * t91961;
    (t91955, t91963)
}
