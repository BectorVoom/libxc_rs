//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 905/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk905(t735: f64, t8115: f64, t2222: f64, t2341: f64, t2240: f64, t72: f64, t732: f64, t186: f64, t660: f64, t755: f64, t730: f64, t2383: f64, t2391: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8117 = 0.51947577317044391277e2_f64 * t735 * t8115;
    let t8118 = t2341 * t2222;
    let t8120 = t2240 * t72;
    let t8121 = t8120 * t732;
    let t8124 = t660 * t755 * t186;
    let t8126 = 0.56968947174242584612e-3_f64 * t730 * t8124;
    let t8127 = t2383 * t2391;
    (t8117, t8118, t8121, t8124, t8126, t8127)
}
