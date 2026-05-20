//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1279/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1279<F: Float>(t3014: F, t4707: F, t972: F, t11450: F, t11461: F, t11466: F, t11554: F, t15100: F, t15103: F, t15104: F, t15235: F, t15238: F, t15242: F, t15249: F, t15252: F, t15255: F, t2945: F, t2968: F, t2987: F, t3012: F, t4690: F, t4712: F, t965: F) -> F {
    let t15258 = t4707 * t3014;
    let t15259 = t15258 * t972;
    let t15262 = -t15100 + t15103 - F::new(2.0) * t15104 * t2945 + F::cast_from(0.5848223622634646207e0_f64) * t965 * t15235 + F::cast_from(0.32163958997385070134e2_f64) * t2968 * t15238 + F::cast_from(0.2069040516770936012e4_f64) * t11450 * t15242 - F::cast_from(0.23392894490538584828e1_f64) * t11554 * t4690 + F::cast_from(0.34631718211362927518e2_f64) * t11461 * t4712 - F::cast_from(0.23392894490538584828e1_f64) * t2987 * t15249 - F::cast_from(0.11696447245269292414e1_f64) * t2987 * t15252 - F::cast_from(0.10389515463408878255e3_f64) * t11466 * t15255 + F::cast_from(0.34631718211362927518e2_f64) * t3012 * t15259;
    t15262
}
