//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1189/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1189(t40: f64, t10121: f64, t870: f64, t2517: f64, t2519: f64, t195: f64, t632: f64, t2244: f64, t2250: f64, t2433: f64, t39097: f64, t39103: f64, t39110: f64, t73: f64, t9258: f64, t9427: f64, t9430: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t146 = t40 <= zeta_threshold;
    let t40622 = t10121 * t870;
    let t40626 = t2519 * t2517;
    let t40627 = 6.0_f64 * t40626;
    let t40632 = 1.0_f64 / t195 / t632;
    let t40645 = piecewise3(t146, 0.0_f64, 40.0_f64 / 81.0_f64 * t40632 * t39097 - 16.0_f64 / 9.0_f64 * t9427 * t2244 * t2250 + 4.0_f64 / 3.0_f64 * t2433 * t39103 + 16.0_f64 / 9.0_f64 * t9430 * t9258 + 4.0_f64 / 3.0_f64 * t73 * t39110);
    (t40622, t40627, t40645)
}
