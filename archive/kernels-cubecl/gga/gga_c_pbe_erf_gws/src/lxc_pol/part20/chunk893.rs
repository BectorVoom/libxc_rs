//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 893/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk893<F: Float>(t353: F, t9915: F, t4386: F, t1109: F, t898: F, t938: F, t859: F, t1105: F, t3306: F, t2376: F, t2409: F, t3060: F, t8589: F) -> (F, F, F, F, F) {
    let t9916 = t353 * t9915;
    let t9917 = t4386 * t9916;
    let t9920 = t898 * t1109;
    let t9921 = t9920 * t938;
    let t9922 = t353 * t9921;
    let t9923 = t859 * t9922;
    let t9926 = t1105 * t3306;
    let t9928 = t2409 * t2376 * t9926;
    let t9932 = t2409 * t8589 * t3060;
    (t9917, t9923, t9926, t9928, t9932)
}
