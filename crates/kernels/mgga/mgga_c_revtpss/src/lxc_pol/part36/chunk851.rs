//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 851/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk851<F: Float>(t1058: F, t6318: F, t247: F, t3109: F, t6096: F, t1063: F, t140: F, t6284: F, t1011: F, t6288: F, t6292: F, t3172: F, t6262: F, t3127: F, t1062: F, t6317: F) -> (F, F, F, F, F, F, F) {
    let t19867 = t6318 * t1058;
    let t19882 = t247 * t3109 * t6096;
    let t19883 = t1063 * t19882;
    let t19900 = t140 * t6284;
    let t19901 = t1011 * t19900;
    let t19907 = t140 * t6288;
    let t19908 = t1011 * t19907;
    let t19912 = t140 * t6292;
    let t19913 = t1011 * t19912;
    let t19920 = t3172 * t6262;
    let t19921 = t3127 * t19920;
    let t19968 = t6317 * t1062;
    (t19867, t19883, t19901, t19908, t19913, t19921, t19968)
}
