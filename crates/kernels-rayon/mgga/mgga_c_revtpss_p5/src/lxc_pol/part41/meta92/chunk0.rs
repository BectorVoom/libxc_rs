//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 525/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk525(t599: f64, t602: f64, t89: f64, t90: f64, t29: f64, t2: f64, t580: f64) -> (f64, f64, f64, f64) {
    let t2242 = t599 * t602;
    let t2246 = 1.0_f64 / t90 / t89;
    let t2247 = t29 * t2246;
    let t2255 = t2 * t580;
    (t2242, t2246, t2247, t2255)
}
