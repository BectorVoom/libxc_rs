//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1352/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1352(t1514: f64, t2289: f64, t4264: f64, t625: f64, t4288: f64, t2349: f64, t97: f64, t105: f64, t2357: f64, t1857: f64, t3857: f64, t177: f64, t5566: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13448 = t2289 * t1514;
    let t13451 = 4.0_f64 / 3.0_f64 * t625 * t4264;
    let t13453 = 2.0_f64 / 3.0_f64 * t625 * t4288;
    let t13475 = t97 * t2349;
    let t13496 = t105 * t2357;
    let t13584 = t3857 * t1857;
    let t13597 = t5566 * t177;
    (t13448, t13451, t13453, t13475, t13496, t13584, t13597)
}
