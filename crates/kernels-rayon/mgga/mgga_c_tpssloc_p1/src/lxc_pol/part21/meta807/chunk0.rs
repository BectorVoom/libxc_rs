//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2810/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2810(t2627: f64, t5631: f64, t13176: f64, t13417: f64, t13431: f64, t13434: f64, t1523: f64, t16823: f64, t17028: f64, t255: f64, t2617: f64, t2633: f64, t4162: f64, t4166: f64, t4296: f64, t4298: f64, t46528: f64, t5648: f64, t5653: f64, t59074: f64, t59230: f64, t812: f64, t860: f64, t9612: f64) -> f64 {
    let t59355 = t2627 * t5631;
    let t59379 = 2.0_f64 * t2633 * t59355 * t812 - t59074 * t812 * t860 - 4.0_f64 * t13176 * t4296 + 4.0_f64 * t13417 * t4166 - 2.0_f64 * t13431 * t4166 - 4.0_f64 * t13434 * t4166 - 2.0_f64 * t1523 * t46528 - 2.0_f64 * t16823 * t2617 - 2.0_f64 * t17028 * t2617 + t255 * t59230 + 4.0_f64 * t4162 * t4298 - 2.0_f64 * t5648 * t9612 - t5653 * t9612;
    t59379
}
