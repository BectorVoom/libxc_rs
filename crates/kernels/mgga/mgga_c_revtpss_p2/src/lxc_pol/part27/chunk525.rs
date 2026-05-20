//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 525/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk525<F: Float>(t2918: F, t935: F, t915: F, t913: F, t275: F, t290: F, t2875: F, t2846: F, t2848: F, t2855: F, t2860: F, t2864: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t2919 = t2918 * t935;
    let t2921 = F::new(1.0) * t915 * t2919;
    let t2922 = t913 * t913;
    let t2923 = F::new(1.0) / t2922;
    let t2924 = t275 * t2923;
    let t2925 = t290 * t290;
    let t2926 = F::new(1.0) / t2925;
    let t2927 = t2875 * t2926;
    let t2929 = F::cast_from(0.16081979498692535067e2_f64) * t2924 * t2927;
    let t2930 = F::cast_from(0.22831111111111111111e-1_f64) * t2846;
    let t2935 = t2930 + F::cast_from(0.11415555555555555555e-1_f64) * t2848 - F::cast_from(0.11415555555555555555e-1_f64) * t2855 + F::cast_from(0.34246666666666666666e-1_f64) * t2860 - F::cast_from(0.17123333333333333333e-1_f64) * t2864;
    (t2919, t2921, t2922, t2923, t2924, t2925, t2926, t2927, t2929, t2935)
}
