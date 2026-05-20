//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1863/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1863<F: Float>(t11134: F, t11366: F, t11422: F, t11423: F, t18948: F, t19002: F, t19004: F, t19007: F, t19009: F, t19014: F, t19017: F, t15123: F, t15125: F, t15301: F, t18906: F, t18911: F, t18915: F, t18919: F, t18924: F, t18928: F, t18932: F, t18934: F, t18939: F, t18951: F, t18980: F, t18982: F, t18985: F, t18988: F, t18990: F, t18993: F, t18995: F, t19202: F) -> F {
    let t19224 = -F::new(0.516475e0) * t18948 - t11422 - t11423 + F::cast_from(0.23154444444444444445e-1_f64) * t19002 - F::cast_from(0.13892666666666666667e0_f64) * t19004 - F::new(0.104195e0) * t19007 + F::cast_from(0.69463333333333333333e-1_f64) * t19009 - F::cast_from(0.22954444444444444444e0_f64) * t11134 - F::cast_from(0.11577222222222222222e0_f64) * t11366 + F::new(0.20839e0) * t19014 - F::cast_from(0.34731666666666666667e-1_f64) * t19017;
    let t19226 = -F::cast_from(0.57386111111111111112e0_f64) * t18906 + F::new(0.20659e1) * t18911 - F::cast_from(0.68863333333333333334e0_f64) * t18915 + F::new(0.6311625e0) * t18951 - F::cast_from(0.23154444444444444445e0_f64) * t15123 - F::cast_from(0.68863333333333333332e0_f64) * t15125 + t15301 - F::new(0.309885e1) * t18928 + F::new(0.20659e1) * t18932 - F::cast_from(0.34431666666666666667e0_f64) * t18939 + t19202 + F::cast_from(0.264729375e1_f64) * t18980 - F::new(0.3529725e1) * t18982 - F::new(0.17648625e1) * t18985 - F::cast_from(0.157790625e0_f64) * t18988 + F::new(0.6311625e0) * t18990 + F::new(0.31558125e0) * t18993 + F::new(0.3529725e1) * t18995 + F::cast_from(0.11477222222222222222e0_f64) * t18919 - F::cast_from(0.34431666666666666667e0_f64) * t18924 + F::cast_from(0.17215833333333333333e0_f64) * t18934 + t19224;
    t19226
}
