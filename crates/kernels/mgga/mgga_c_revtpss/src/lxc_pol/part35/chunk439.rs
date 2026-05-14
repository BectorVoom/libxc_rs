//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 439/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk439<F: Float>(t307: F, t944: F, t302: F, t2846: F, t2904: F, t310: F, t320: F, t963: F, t315: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
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
    let t2985 = t963 * t320;
    let t2986 = 1.0 / t2985;
    let t2987 = t315 * t2986;
    let t2994 = 0.40256666666666666667e0 * t2846;
    let t3001 = 0.137975e0 * t2904;
    let t3010 = t963 * t963;
    let t3011 = 1.0 / t3010;
    (t2942, t2943, t2950, t2957, t2966, t2967, t2968, t2969, t2970, t2974, t2986, t2987, t2994, t3001, t3010, t3011)
}
