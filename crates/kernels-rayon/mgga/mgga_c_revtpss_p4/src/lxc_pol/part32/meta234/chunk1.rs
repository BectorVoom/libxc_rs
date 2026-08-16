//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 997/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk997(t373: f64, t6305: f64, t3155: f64, t1042: f64, t3162: f64, t225: f64, t6235: f64, t366: f64, t1066: f64, t6100: f64, t247: f64, t3182: f64, t6092: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6306 = t373 * t6305;
    let t6307 = t6306 * t3155;
    let t6308 = t1042 * t6307;
    let t6311 = t6306 * t3162;
    let t6312 = t1042 * t6311;
    let t6317 = t6235 * t225;
    let t6318 = t6317 * t366;
    let t6322 = t1066 * t6100;
    let t6323 = t247 * t6322;
    let t6326 = t3182 * t6092;
    (t6307, t6308, t6311, t6312, t6317, t6318, t6323, t6326)
}
