//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 471/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk471<F: Float>(t2845: F, t2970: F, t26: F, t2850: F, t945: F, t2829: F, t2919: F, t2922: F, t2925: F, t2928: F, t2945: F, t2953: F, t2955: F, t2961: F, t2963: F, t2967: F, t2968: F) -> (F, F, F, F, F, F, F) {
    let t2971 = t2970 * t2845;
    let t2972 = t26 * t2971;
    let t2974 = t945 * t2850;
    let t2975 = t26 * t2974;
    let t2977 = t945 * t2829;
    let t2978 = t26 * t2977;
    let t2980 = -F::cast_from(0.9494625e0_f64) * t2945 + F::cast_from(0.1898925e1_f64) * t2953 + t2955 + F::cast_from(0.19931111111111111111e0_f64) * t2919 - F::cast_from(0.19931111111111111111e0_f64) * t2922 + F::cast_from(0.59793333333333333334e0_f64) * t2925 - F::cast_from(0.29896666666666666667e0_f64) * t2928 + F::cast_from(0.15358125e0_f64) * t2961 + F::cast_from(0.3071625e0_f64) * t2963 + t2967 + F::cast_from(0.10954222222222222222e0_f64) * t2968 - F::cast_from(0.27385555555555555556e-1_f64) * t2972 + F::cast_from(0.16431333333333333333e0_f64) * t2975 - F::cast_from(0.82156666666666666667e-1_f64) * t2978;
    (t2971, t2972, t2974, t2975, t2977, t2978, t2980)
}
