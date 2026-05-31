//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 633/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk633<F: Float>(t290: F, t2875: F, t2924: F, t2846: F, t2848: F, t2855: F, t2860: F, t2864: F, t941: F, t945: F, t307: F, t944: F) -> (F, F, F, F, F, F, F, F) {
    let t2925 = t290 * t290;
    let t2926 = F::cast_from(1.0_f64) / t2925;
    let t2927 = t2875 * t2926;
    let t2929 = F::cast_from(0.16081979498692535067e2_f64) * t2924 * t2927;
    let t2930 = F::cast_from(0.22831111111111111111e-1_f64) * t2846;
    let t2935 = t2930 + F::cast_from(0.11415555555555555555e-1_f64) * t2848 - F::cast_from(0.11415555555555555555e-1_f64) * t2855 + F::cast_from(0.34246666666666666666e-1_f64) * t2860 - F::cast_from(0.17123333333333333333e-1_f64) * t2864;
    let t2938 = t941 * t945;
    let t2941 = t944 * t307;
    (t2925, t2926, t2927, t2929, t2930, t2935, t2938, t2941)
}
