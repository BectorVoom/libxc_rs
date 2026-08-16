//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2207/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2207(t16985: f64, t2697: f64, t1516: f64, t47275: f64, t47278: f64, t5628: f64, t9601: f64, t5619: f64, t9671: f64, t16853: f64, t16673: f64, t2638: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t59257 = t2697 * t16985;
    let t59259 = t47275 * t1516;
    let t59261 = t47278 * t1516;
    let t59263 = t9601 * t5628;
    let t59276 = t9671 * t5619;
    let t59279 = t2697 * t16853;
    let t59281 = t16673 * t2638;
    (t59257, t59259, t59261, t59263, t59276, t59279, t59281)
}
