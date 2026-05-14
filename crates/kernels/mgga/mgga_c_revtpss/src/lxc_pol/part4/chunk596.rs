//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 596/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk596<F: Float>(t290: F, t2875: F, t2924: F, t2846: F, t2848: F, t2855: F, t2860: F, t2864: F, t941: F, t945: F, t307: F, t944: F, t302: F, t953: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t2925 = t290 * t290;
    let t2926 = 1.0 / t2925;
    let t2927 = t2875 * t2926;
    let t2929 = 0.16081979498692535067e2 * t2924 * t2927;
    let t2930 = 0.22831111111111111111e-1 * t2846;
    let t2935 = t2930 + 0.11415555555555555555e-1 * t2848 - 0.11415555555555555555e-1 * t2855 + 0.34246666666666666666e-1 * t2860 - 0.17123333333333333333e-1 * t2864;
    let t2938 = t941 * t945;
    let t2941 = t944 * t307;
    let t2942 = 1.0 / t2941;
    let t2943 = t302 * t2942;
    let t2944 = t953 * t953;
    (t2925, t2926, t2927, t2929, t2930, t2935, t2938, t2942, t2943, t2944)
}
