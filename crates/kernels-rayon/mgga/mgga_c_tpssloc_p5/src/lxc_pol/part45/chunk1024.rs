//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 1024/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk1024(t114223: f64, t114225: f64, t114230: f64, t114234: f64, t114241: f64, t114243: f64, t114247: f64, t114254: f64, t114256: f64, t114262: f64, t115572: f64, t115577: f64, t115583: f64, t115586: f64, t22653: f64, t22913: f64, t31601: f64, t3758: f64, t7194: f64) -> f64 {
    let t115590 = t114223 + 0.82246703342411321824e-2_f64 * t115572 + t114225 + t114230 + t114234 - t114241 - t114243 - t114247 + 4.0_f64 * t3758 * t31601 - 0.82246703342411321825e-2_f64 * t115577 + t114254 - t114256 + 4.0_f64 * t7194 * t22653 + 0.16449340668482264365e-1_f64 * t115583 - 0.16449340668482264365e-1_f64 * t115586 + 2.0_f64 * t7194 * t22913 - t114262;
    t115590
}
