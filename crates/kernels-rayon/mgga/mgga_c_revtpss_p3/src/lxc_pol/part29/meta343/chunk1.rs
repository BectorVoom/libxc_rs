//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1265/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1265(t11922: f64, t3119: f64, t3115: f64, t1086: f64, t3057: f64, t3090: f64, t11671: f64, t3114: f64, t127: f64, t3206: f64, t371: f64, t3205: f64) -> (f64, f64, f64, f64) {
    let t11923 = t11922 * t3119;
    let t11924 = t3115 * t11923;
    let t11926 = t3057 * t1086;
    let t11927 = t11926 * t3090;
    let t11933 = t3114 * t11671;
    let t11937 = t371 * t127 * t3206;
    let t11938 = t3205 * t11937;
    (t11924, t11927, t11933, t11938)
}
