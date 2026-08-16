//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2214/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2214(t27932: f64, t74477: f64, t74419: f64, t98196: f64, t74423: f64, t22021: f64, t25986: f64, t2661: f64, t22068: f64, t25972: f64, t25978: f64, t6880: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t108615 = t27932 * t74477;
    let t108617 = t98196 * t74419;
    let t108619 = t27932 * t74423;
    let t108623 = t2661 * t25986 * t22021;
    let t108625 = t25972 * t22068;
    let t108627 = t25978 * t6880;
    (t108615, t108617, t108619, t108623, t108625, t108627)
}
