//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1451/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1451(t41491: f64, t983: f64, t11502: f64, t11601: f64, t981: f64, t2922: f64, t275: f64, t2875: f64, t2925: f64, t11506: f64, t15542: f64, t3006: f64) -> (f64, f64, f64, f64, f64) {
    let t41493 = 0.23392894490538584828e1_f64 * t41491 * t983;
    let t41496 = 0.46785788981077169656e1_f64 * t981 * t11601 * t11502;
    let t41497 = t2922 * t2922;
    let t41499 = t275 / t41497;
    let t41500 = t2875 * t2875;
    let t41501 = t2925 * t2925;
    let t41502 = 1.0_f64 / t41501;
    let t41505 = 0.24955700379505800916e5_f64 * t41499 * t41500 * t41502;
    let t41509 = 0.61524113149298439947e4_f64 * t981 * t11506 * t3006 * t15542;
    (t41493, t41496, t41500, t41505, t41509)
}
