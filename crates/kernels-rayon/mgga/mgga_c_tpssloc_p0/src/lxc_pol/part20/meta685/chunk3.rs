//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2598/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2598(t15495: f64, t3572: f64, t1227: f64, t1653: f64, t248: f64, t45293: f64, t15591: f64, t15643: f64, t3490: f64, t1089: f64, t3507: f64, t607: f64) -> (f64, f64, f64, f64, f64) {
    let t52674 = t15495 * t3572;
    let t52680 = t1227 * t248 * t45293 * t1653;
    let t52682 = t15591 * t3572;
    let t52684 = t3490 * t15643;
    let t52687 = t3507 * t1089 * t607;
    (t52674, t52680, t52682, t52684, t52687)
}
