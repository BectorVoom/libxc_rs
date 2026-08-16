//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 915/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk915(t1337: f64, t9586: f64, t4135: f64, t5541: f64, t7315: f64, t9514: f64, t9517: f64, t9521: f64, t9560: f64, t9562: f64, t9565: f64, t9567: f64, t9569: f64, t9571: f64, t9574: f64, t9577: f64, t9579: f64, t9581: f64) -> (f64, f64) {
    let t9588 = 0.56968947174242584612e-3_f64 * t1337 * t9586;
    let t9589 = -3.0_f64 * t4135 * t5541 * t7315 + t9514 - t9517 - t9521 + t9560 + t9562 - t9565 + t9567 + t9569 - t9571 - t9574 - t9577 + t9579 - t9581 - t9588;
    (t9588, t9589)
}
