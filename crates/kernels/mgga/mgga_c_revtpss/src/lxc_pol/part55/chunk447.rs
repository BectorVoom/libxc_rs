//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 447/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk447<F: Float>(t2904: F, t698: F, t931: F, t1014: F, t240: F, t913: F, t275: F, t290: F, t2846: F, t941: F, t945: F, t307: F, t944: F, t302: F, t310: F, t960: F, t964: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t2905 = 0.13692777777777777778e0 * t2904;
    let t2906 = t698 * t931;
    let t2908 = t240 * t1014;
    let t2922 = t913 * t913;
    let t2923 = 1.0 / t2922;
    let t2924 = t275 * t2923;
    let t2925 = t290 * t290;
    let t2926 = 1.0 / t2925;
    let t2930 = 0.22831111111111111111e-1 * t2846;
    let t2938 = t941 * t945;
    let t2941 = t944 * t307;
    let t2942 = 1.0 / t2941;
    let t2943 = t302 * t2942;
    let t2950 = 0.68863333333333333333e0 * t2846;
    let t2957 = 0.17365833333333333333e0 * t2904;
    let t2966 = t944 * t944;
    let t2967 = 1.0 / t2966;
    let t2968 = t302 * t2967;
    let t2969 = t310 * t310;
    let t2970 = 1.0 / t2969;
    let t2974 = 0.12361111111111111111e-1 * t2846;
    let t2982 = t960 * t964;
    (t2905, t2906, t2908, t2924, t2926, t2930, t2938, t2943, t2950, t2957, t2968, t2970, t2974, t2982)
}
