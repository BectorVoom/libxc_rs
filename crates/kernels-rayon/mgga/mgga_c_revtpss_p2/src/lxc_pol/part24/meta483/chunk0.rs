//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1474/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1474(t3671: f64, t371: f64, t6609: f64, t676: f64, t480: f64, t69637: f64, t17303: f64, t5323: f64, t5327: f64, t1284: f64, t20849: f64, t3624: f64) -> (f64, f64, f64, f64, f64) {
    let t70511 = t3671 * t371 * t676 * t6609;
    let t70578 = t69637 * t480;
    let t70583 = t5323 * t17303;
    let t70758 = t5327 * t17303;
    let t70800 = t20849 * t1284 * t3624;
    (t70511, t70578, t70583, t70758, t70800)
}
