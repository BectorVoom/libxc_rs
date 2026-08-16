//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1035/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1035(t32591: f64, t606: f64, t8442: f64, t1925: f64, t84: f64, t640: f64, t8621: f64, t7002: f64, t93: f64, t1419: f64, t3140: f64, t8477: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t32592 = t32591 * t606;
    let t32593 = t8442 * t32592;
    let t32600 = t84 * t1925;
    let t32602 = t8621 * t32600 * t640;
    let t32655 = t93 * t7002;
    let t32699 = t1419 * t3140;
    let t32700 = t8477 * t32699;
    (t32593, t32600, t32602, t32655, t32699, t32700)
}
