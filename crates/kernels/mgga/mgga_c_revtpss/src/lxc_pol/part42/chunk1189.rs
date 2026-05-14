//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1189/1363 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1189<F: Float>(t19691: F, t4801: F, t1042: F, t140: F, t6284: F, t1011: F, t6288: F, t6292: F, t1015: F, t18281: F, t1012: F, t3172: F, t6262: F, t3127: F, t11881: F, t15986: F, t15990: F, t15996: F, t16037: F, t3241: F, t6289: F, t6293: F) -> (F, F, F) {
    let t19894 = t4801 * t19691;
    let t19895 = t1042 * t19894;
    let t19900 = t140 * t6284;
    let t19901 = t1011 * t19900;
    let t19907 = t140 * t6288;
    let t19908 = t1011 * t19907;
    let t19912 = t140 * t6292;
    let t19913 = t1011 * t19912;
    let t19916 = t1015 * t18281;
    let t19917 = t1012 * t19916;
    let t19920 = t3172 * t6262;
    let t19921 = t3127 * t19920;
    let t19923 = -t3241 * t6289 / 108.0 + t19908 / 864.0 - t3241 * t6293 / 81.0 + t19913 / 648.0 - t11881 / 1296.0 + t15986 - t15990 + t15996 - t16037 + t1011 * t19917 / 288.0 - 0.19055119163586549765e-3 * t19921;
    (t19895, t19901, t19923)
}
