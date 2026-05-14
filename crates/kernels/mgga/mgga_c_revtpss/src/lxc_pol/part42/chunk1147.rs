//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1147/1363 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1147<F: Float>(t141: F, t19006: F, t6138: F, t698: F, t18942: F, t930: F, t18937: F, t2908: F, t11134: F, t11366: F, t11479: F, t11480: F, t18948: F, t19002: F, t19004: F, t15123: F, t15125: F, t15128: F, t18906: F, t18911: F, t18915: F, t18919: F, t18924: F, t18928: F, t18932: F, t18934: F, t18939: F, t18951: F, t18977: F, t18980: F, t18982: F, t18985: F, t18988: F, t18990: F, t18993: F, t18995: F) -> (F, F, F, F, F) {
    let t19007 = t141 * t19006;
    let t19009 = t698 * t6138;
    let t19013 = t930 * t18942;
    let t19014 = t141 * t19013;
    let t19016 = t2908 * t18937;
    let t19017 = t141 * t19016;
    let t19019 = -0.301925e0 * t18948 - t11479 - t11480 + 0.18396666666666666667e-1 * t19002 - 0.11038e0 * t19004 - 0.82785e-1 * t19007 + 0.5519e-1 * t19009 - 0.13418888888888888889e0 * t11134 - 0.91983333333333333333e-1 * t11366 + 0.16557e0 * t19014 - 0.27595e-1 * t19017;
    let t19021 = -0.33547222222222222222e0 * t18906 + 0.12077e1 * t18911 - 0.40256666666666666666e0 * t18915 + 0.16504875e0 * t18951 - 0.18396666666666666667e0 * t15123 - 0.40256666666666666668e0 * t15125 + t15128 - 0.181155e1 * t18928 + 0.12077e1 * t18932 - 0.20128333333333333333e0 * t18939 + t18977 + 0.19419375e1 * t18980 - 0.258925e1 * t18982 - 0.1294625e1 * t18985 - 0.412621875e-1 * t18988 + 0.16504875e0 * t18990 + 0.82524375e-1 * t18993 + 0.258925e1 * t18995 + 0.67094444444444444443e-1 * t18919 - 0.20128333333333333333e0 * t18924 + 0.10064166666666666667e0 * t18934 + t19019;
    (t19007, t19009, t19014, t19017, t19021)
}
