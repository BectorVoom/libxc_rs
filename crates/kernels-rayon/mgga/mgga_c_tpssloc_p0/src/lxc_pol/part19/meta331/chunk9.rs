//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1190/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1190(t52: f64, t197: f64, t636: f64, t2244: f64, t2250: f64, t2440: f64, t39097: f64, t39103: f64, t39110: f64, t76: f64, t9258: f64, t9438: f64, t9441: f64, zeta_threshold: f64) -> f64 {
    let t150 = t52 <= zeta_threshold;
    let t40647 = 1.0_f64 / t197 / t636;
    let t40660 = piecewise3(t150, 0.0_f64, 40.0_f64 / 81.0_f64 * t40647 * t39097 + 16.0_f64 / 9.0_f64 * t9438 * t2244 * t2250 + 4.0_f64 / 3.0_f64 * t2440 * t39103 + 16.0_f64 / 9.0_f64 * t9441 * t9258 - 4.0_f64 / 3.0_f64 * t76 * t39110);
    t40660
}
