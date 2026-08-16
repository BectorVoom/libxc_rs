//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 883/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk883(t125: f64, t755: f64, t123: f64, t128: f64, t121: f64, t22: f64, t2196: f64, t7823: f64, t667: f64, t7826: f64, t124: f64, t138: f64, t599: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7829 = t125 * t755;
    let t7830 = t123 * t7829;
    let t7832 = 1.0_f64/pow_3_2(t128);
    let t7833 = t7832 * t121;
    let t7834 = t7833 * t22;
    let t7836 = t2196 * t7823;
    let t7838 = t667 * t7826;
    let t7841 = t138 * t124 * t599;
    (t7829, t7830, t7834, t7836, t7838, t7841)
}
