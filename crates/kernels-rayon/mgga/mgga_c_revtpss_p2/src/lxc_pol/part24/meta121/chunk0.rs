//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 658/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk658(t4398: f64, t762: f64, t162: f64, t2611: f64, t227: f64, t73: f64, t1544: f64, t853: f64, t1559: f64, t221: f64, t2485: f64, t2484: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4399 = t4398 * t762;
    let t4401 = t2611 * t162;
    let t4415 = t227 * t73;
    let t4416 = t853 * t1544;
    let t4430 = t2485 * t221 * t1559;
    let t4431 = t2484 * t4430;
    (t4399, t4401, t4415, t4416, t4430, t4431)
}
