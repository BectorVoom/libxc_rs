//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 533/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk533(t2435: f64, t781: f64, t124: f64, t68: f64, t138: f64) -> (f64, f64, f64) {
    let t2437 = 0.73171657588172351096e-2_f64 * t2435 * t781;
    let t2438 = t124 * t68;
    let t2439 = t138 * t2438;
    (t2437, t2438, t2439)
}
