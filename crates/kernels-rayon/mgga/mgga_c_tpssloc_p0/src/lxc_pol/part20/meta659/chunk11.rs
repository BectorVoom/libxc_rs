//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2463/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2463(t3185: f64, t49649: f64, t11031: f64, t11054: f64, t11081: f64, t14578: f64, t14596: f64, t14605: f64, t14608: f64, t14622: f64, t1629: f64, t1630: f64, t3076: f64, t3131: f64, t3180: f64, t3186: f64, t3189: f64, t3200: f64, t43473: f64, t43515: f64, t43542: f64, t4669: f64, t4680: f64, t4684: f64, t4691: f64, t47819: f64) -> f64 {
    let t50465 = t49649 * t3185;
    let t50490 = 14.0_f64 * t1629 * t3131 * t43515 * t47819 + 6.0_f64 * t11054 * t3186 * t4680 - 3.0_f64 * t14605 * t3200 * t4684 - 3.0_f64 * t14622 * t3200 * t4680 + 3.0_f64 * t11031 * t4669 - 3.0_f64 * t11081 * t14608 + 18.0_f64 * t14578 * t43473 + 3.0_f64 * t14596 * t3180 + t1630 * t43542 + 3.0_f64 * t3076 * t4691 + 6.0_f64 * t3189 * t50465;
    t50490
}
