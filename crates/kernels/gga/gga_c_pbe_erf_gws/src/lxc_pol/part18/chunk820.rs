//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 820/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk820<F: Float>(t353: F, t9921: F, t859: F, t1105: F, t3306: F, t2376: F, t2409: F, t3060: F, t8589: F, t2416: F, t3068: F, t9283: F, t2362: F, t2397: F, t2408: F, t2498: F, t2503: F, t3052: F, t3733: F, t3921: F, t6778: F, t827: F, t8629: F, t8654: F, t8671: F, t8677: F, t8790: F, t8793: F, t9726: F, t9729: F, t9899: F, t9902: F, t9907: F, t9912: F, t9917: F) -> (F, F, F, F, F, F) {
    let t9922 = t353 * t9921;
    let t9923 = t859 * t9922;
    let t9926 = t1105 * t3306;
    let t9928 = t2409 * t2376 * t9926;
    let t9932 = t2409 * t8589 * t3060;
    let t9941 = t2416 * t1105;
    let t9942 = t9941 * t3068;
    let t9943 = t9283 * t9942;
    let t9946 = -t8671 - t9726 * t3733 / 96.0 - t827 * t9899 / 96.0 - t9902 * t2362 / 48.0 - t9729 * t3733 / 96.0 + t9907 * t6778 / 48.0 + t8677 + t2498 * t2503 / 48.0 + 7.0 / 72.0 * t9912 + t8629 * t9917 / 48.0 + t8629 * t9923 / 96.0 + t2408 * t9928 / 24.0 + t2408 * t9932 / 24.0 + t8793 * t8790 / 24.0 + t3921 * t2397 / 96.0 - t8654 * t3052 / 24.0 - t2408 * t9943 / 12.0;
    (t9923, t9926, t9928, t9932, t9942, t9946)
}
