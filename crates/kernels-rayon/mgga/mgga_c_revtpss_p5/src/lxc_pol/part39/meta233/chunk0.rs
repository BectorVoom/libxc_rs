//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 904/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk904(t1796: f64, t3172: f64, t1247: f64, t1263: f64, t3367: f64, t4181: f64, t1042: f64, t1032: f64, t1770: f64, t1246: f64, t1774: f64, t1122: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5265 = t3172 * t1796;
    let t5266 = t1247 * t5265;
    let t5268 = t1263 * t3367;
    let t5269 = t5268 * t4181;
    let t5270 = t1042 * t5269;
    let t5273 = t1770 * t1032;
    let t5274 = t5273 * t1246;
    let t5277 = t1263 * t1774;
    let t5278 = t5277 * t1122;
    (t5265, t5266, t5268, t5269, t5270, t5274, t5277, t5278)
}
