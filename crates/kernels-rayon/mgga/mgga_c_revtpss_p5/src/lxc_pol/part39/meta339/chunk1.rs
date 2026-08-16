//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1137/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1137(t13458: f64, t665: f64, t2366: f64, t4263: f64, t10227: f64, t1504: f64, t2350: f64, t2349: f64, t97: f64, t2255: f64, t658: f64, t2256: f64, t4269: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13459 = t13458 * t665;
    let t13462 = t4263 * t2366;
    let t13472 = t10227 * t1504 * t2350;
    let t13475 = t97 * t2349;
    let t13476 = t2255 * t658;
    let t13479 = t4269 * t2256;
    (t13459, t13462, t13472, t13475, t13476, t13479)
}
