//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1516/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1516(t23210: f64, t705: f64, t221: f64, t23245: f64, t2484: f64, t2485: f64, t23168: f64, t40352: f64, t1568: f64, t6016: f64, t231: f64, t2782: f64, t2783: f64) -> (f64, f64, f64, f64, f64) {
    let t77054 = t705 * t23210;
    let t77127 = t2484 * t2485 * t221 * t23245;
    let t77131 = t40352 * t2485 * t221 * t23168;
    let t77159 = t1568 * t6016;
    let t77171 = t2782 * t2783 * t77159 * t231;
    (t77054, t77127, t77131, t77159, t77171)
}
