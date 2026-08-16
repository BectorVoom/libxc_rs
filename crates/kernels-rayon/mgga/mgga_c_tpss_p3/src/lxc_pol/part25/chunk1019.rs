//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1019/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1019(t57: f64, t2232: f64, t4579: f64, t13335: f64, t14096: f64, t3431: f64, t3582: f64, t581: f64, t81: f64, t14095: f64, t162: f64, t187: f64, t8101: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t155 = t57 <= zeta_threshold;
    let t14101 = t2232 * t4579;
    let t14107 = piecewise3(t155, 0.0_f64, 8.0_f64 / 27.0_f64 * t14096 * t581 + 8.0_f64 / 9.0_f64 * t3582 * t3431 + 4.0_f64 / 9.0_f64 * t14101 * t581 - 4.0_f64 / 3.0_f64 * t81 * t13335);
    let t14108 = t14095 + t14107;
    let t14109 = t14108 * t162;
    let t14111 = 0.19751673498613801407e-1_f64 * t14109 * t187;
    let t14112 = 4.0_f64 * t8101;
    (t14108, t14111, t14112)
}
