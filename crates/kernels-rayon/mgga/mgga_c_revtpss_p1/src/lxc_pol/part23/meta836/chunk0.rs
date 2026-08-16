//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2707/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2707(t3520: f64, t6513: f64, t3495: f64, t3476: f64, t6481: f64, t20520: f64, t3479: f64, t3451: f64, t20382: f64, t3523: f64, t12555: f64, t6534: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t69359 = t6513 * t3520;
    let t69371 = t6513 * t3495;
    let t69376 = t6481 * t3476;
    let t69411 = t20520 * t3479;
    let t69488 = t6481 * t3451;
    let t69504 = t20382 * t3523;
    let t69511 = t6534 * t12555;
    (t69359, t69371, t69376, t69411, t69488, t69504, t69511)
}
