//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 463/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk463<F: Float>(t340: F, t992: F, t338: F, t378: F, t1071: F, t994: F, t2846: F, t221: F, t346: F, t696: F, t345: F, t1003: F, t1007: F) -> (F, F, F, F, F, F) {
    let t3056 = F::cast_from(1.0_f64) / t992 / t340;
    let t3057 = t338 * t3056;
    let t3058 = t3057 * t378;
    let t3063 = t994 * t1071;
    let t3070 = F::cast_from(0.19755555555555555556e-1_f64) * t2846;
    let t3080 = t221 * t696 * t346;
    let t3082 = t345 * t3080 / F::cast_from(432.0_f64);
    let t3086 = t1003 * t1007;
    (t3057, t3058, t3063, t3070, t3082, t3086)
}
