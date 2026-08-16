//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 440/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk440(t2281: f64, t2262: f64, t2268: f64, t2271: f64, t2275: f64, t2278: f64, t39: f64, t44: f64, t51: f64, t615: f64, t618: f64, t33: f64) -> (f64, f64) {
    let t2282 = 88.0_f64 / 9.0_f64 * t2281;
    let t2283 = 88.0_f64 / 9.0_f64 * t2262 * t44 - 40.0_f64 / 9.0_f64 * t615 * t618 + 5.0_f64 / 18.0_f64 * t39 * t2268 + 5.0_f64 / 6.0_f64 * t39 * t2271 + 5.0_f64 / 18.0_f64 * t51 * t2275 - 5.0_f64 / 6.0_f64 * t51 * t2278 - t2282;
    let t2284 = t33 * t2283;
    (t2283, t2284)
}
