//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 905/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk905(t1514: f64, t625: f64, t1513: f64, t2339: f64, t665: f64, t1504: f64, t2349: f64, t658: f64, t100: f64, t2: f64, t580: f64, t1509: f64, t2357: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4261 = t625 * t1514;
    let t4263 = t2339 * t1513;
    let t4264 = t4263 * t665;
    let t4269 = t2349 * t1504;
    let t4270 = t4269 * t658;
    let t4273 = t100 * t2;
    let t4274 = t4273 * t580;
    let t4279 = t2357 * t1509;
    (t4261, t4263, t4264, t4269, t4270, t4273, t4274, t4279)
}
