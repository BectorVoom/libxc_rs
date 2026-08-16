//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2295/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2295(t10055: f64, t13380: f64, t13384: f64, t13385: f64, t13407: f64, t13414: f64, t13434: f64, t13453: f64, t25236: f64, t2613: f64, t2617: f64, t2679: f64, t4166: f64, t4281: f64, t4286: f64, t4291: f64, t4298: f64, t47425: f64, t829: f64, t9612: f64, t9632: f64) -> f64 {
    let t47507 = 6.0_f64 * t13380 * t4281 * t9632 + 6.0_f64 * t13384 * t4281 * t9632 - 3.0_f64 * t25236 * t2679 * t4291 - 3.0_f64 * t4291 * t47425 * t829 + 6.0_f64 * t10055 * t4166 + 12.0_f64 * t13385 * t13453 - 6.0_f64 * t13407 * t2617 - 3.0_f64 * t13414 * t2617 - 6.0_f64 * t13434 * t2617 + 3.0_f64 * t2613 * t4298 - 3.0_f64 * t4286 * t9612;
    t47507
}
