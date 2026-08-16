//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 213/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk213(t241: f64, t550: f64, t247: f64, t217: f64, t535: f64, t548: f64) -> f64 {
    let t551 = t241 * t550;
    let t552 = t551 * t247;
    let t555 = t217 * t535 / 96.0_f64 + 0.21437009059034868486e-3_f64 * t548 * t552;
    t555
}
