//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1856/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1856<F: Float>(t11134: F, t11534: F, t15127: F, t15189: F, t15503: F, t15504: F, t18906: F, t18911: F, t18915: F, t18919: F, t18924: F, t18928: F, t18932: F, t18934: F, t18939: F, t18944: F, t18948: F) -> F {
    let t19077 = -t11534 - F::cast_from(0.79148148148148148147e-2_f64) * t11134 - F::cast_from(0.15829629629629629629e-1_f64) * t15189 + F::cast_from(0.79148148148148148147e-2_f64) * t15127 - t15503 + t15504 + F::cast_from(0.39574074074074074073e-2_f64) * t18919 - F::cast_from(0.19787037037037037037e-1_f64) * t18906 + F::cast_from(0.71233333333333333332e-1_f64) * t18911 - F::cast_from(0.23744444444444444444e-1_f64) * t18915 - F::cast_from(0.11872222222222222222e-1_f64) * t18924 - F::cast_from(0.10685e0_f64) * t18928 + F::cast_from(0.71233333333333333332e-1_f64) * t18932 + F::cast_from(0.5936111111111111111e-2_f64) * t18934 - F::cast_from(0.11872222222222222222e-1_f64) * t18939 + F::cast_from(0.35616666666666666666e-1_f64) * t18944 - F::cast_from(0.17808333333333333333e-1_f64) * t18948;
    t19077
}
