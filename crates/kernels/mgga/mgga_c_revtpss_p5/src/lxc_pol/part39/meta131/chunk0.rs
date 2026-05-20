//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 636/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk636<F: Float>(t2962: F, t954: F, t944: F, t302: F, t310: F, t2944: F, t2846: F, t2848: F, t2855: F, t2860: F, t2864: F, t324: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t2963 = t2962 * t954;
    let t2966 = t944 * t944;
    let t2967 = F::new(1.0) / t2966;
    let t2968 = t302 * t2967;
    let t2969 = t310 * t310;
    let t2970 = F::new(1.0) / t2969;
    let t2971 = t2944 * t2970;
    let t2974 = F::cast_from(0.12361111111111111111e-1_f64) * t2846;
    let t2979 = t2974 + F::cast_from(0.61805555555555555556e-2_f64) * t2848 - F::cast_from(0.61805555555555555555e-2_f64) * t2855 + F::cast_from(0.18541666666666666667e-1_f64) * t2860 - F::cast_from(0.92708333333333333333e-2_f64) * t2864;
    let t2980 = t2979 * t324;
    (t2963, t2966, t2967, t2968, t2969, t2970, t2971, t2974, t2979, t2980)
}
