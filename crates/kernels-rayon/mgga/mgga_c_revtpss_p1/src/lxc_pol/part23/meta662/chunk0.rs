//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2393/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2393(t275: f64, t2872: f64, t2922: f64, t41245: f64, t41306: f64, t315: f64, t41235: f64, t11449: f64, t941: f64, t2941: f64, t2966: f64, t302: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41588 = t275 / t2922 / t2872;
    let t41592 = 0.13388493827160493828e1_f64 * t41245;
    let t41610 = 0.31003950617283950618e1_f64 * t41306;
    let t41658 = t315 * t41235;
    let t41662 = t941 * t11449;
    let t41667 = t302 / t2966 / t2941;
    (t41588, t41592, t41610, t41658, t41662, t41667)
}
