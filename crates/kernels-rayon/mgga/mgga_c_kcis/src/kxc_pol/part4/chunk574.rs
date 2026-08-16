//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 574/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk574(t2845: f64, t2970: f64, t26: f64, t2850: f64, t945: f64, t2829: f64, t2919: f64, t2922: f64, t2925: f64, t2928: f64, t2945: f64, t2953: f64, t2955: f64, t2961: f64, t2963: f64, t2967: f64, t2968: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2971 = t2970 * t2845;
    let t2972 = t26 * t2971;
    let t2974 = t945 * t2850;
    let t2975 = t26 * t2974;
    let t2977 = t945 * t2829;
    let t2978 = t26 * t2977;
    let t2980 = -0.9494625e0_f64 * t2945 + 0.1898925e1_f64 * t2953 + t2955 + 0.19931111111111111111e0_f64 * t2919 - 0.19931111111111111111e0_f64 * t2922 + 0.59793333333333333334e0_f64 * t2925 - 0.29896666666666666667e0_f64 * t2928 + 0.15358125e0_f64 * t2961 + 0.3071625e0_f64 * t2963 + t2967 + 0.10954222222222222222e0_f64 * t2968 - 0.27385555555555555556e-1_f64 * t2972 + 0.16431333333333333333e0_f64 * t2975 - 0.82156666666666666667e-1_f64 * t2978;
    (t2971, t2972, t2974, t2975, t2977, t2978, t2980)
}
