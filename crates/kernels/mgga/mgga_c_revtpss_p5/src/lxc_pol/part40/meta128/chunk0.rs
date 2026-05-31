//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 632/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk632<F: Float>(t2853: F, t2908: F, t141: F, t2858: F, t930: F, t2862: F, t2848: F, t2855: F, t2860: F, t2864: F, t2882: F, t2890: F, t2892: F, t2898: F, t2900: F, t2905: F, t2906: F) -> (F, F, F, F, F, F, F) {
    let t2909 = t2908 * t2853;
    let t2910 = t141 * t2909;
    let t2912 = t930 * t2858;
    let t2913 = t141 * t2912;
    let t2915 = t930 * t2862;
    let t2916 = t141 * t2915;
    let t2918 = -F::cast_from(0.9494625e0_f64) * t2882 + F::cast_from(0.1898925e1_f64) * t2890 + t2892 + F::cast_from(0.19931111111111111111e0_f64) * t2848 - F::cast_from(0.19931111111111111111e0_f64) * t2855 + F::cast_from(0.59793333333333333334e0_f64) * t2860 - F::cast_from(0.29896666666666666667e0_f64) * t2864 + F::cast_from(0.15358125e0_f64) * t2898 + F::cast_from(0.3071625e0_f64) * t2900 + t2905 + F::cast_from(0.10954222222222222222e0_f64) * t2906 - F::cast_from(0.27385555555555555556e-1_f64) * t2910 + F::cast_from(0.16431333333333333333e0_f64) * t2913 - F::cast_from(0.82156666666666666667e-1_f64) * t2916;
    (t2909, t2910, t2912, t2913, t2915, t2916, t2918)
}
