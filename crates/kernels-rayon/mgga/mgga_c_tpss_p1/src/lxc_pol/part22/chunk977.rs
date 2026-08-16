//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 977/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk977(t57: f64, t1289: f64, t8061: f64, t2232: f64, t3431: f64, t10353: f64, t1985: f64, t1992: f64, t3582: f64, t581: f64, t81: f64, t10484: f64, t162: f64, zeta_threshold: f64) -> (f64, f64) {
    let t155 = t57 <= zeta_threshold;
    let t10485 = t8061 * t1289;
    let t10488 = t2232 * t3431;
    let t10496 = piecewise3(t155, 0.0_f64, 8.0_f64 / 27.0_f64 * t10485 * t1985 + 8.0_f64 / 9.0_f64 * t10488 * t581 + 4.0_f64 / 9.0_f64 * t3582 * t1992 - 4.0_f64 / 3.0_f64 * t81 * t10353);
    let t10497 = t10484 + t10496;
    let t10498 = t10497 * t162;
    (t10497, t10498)
}
