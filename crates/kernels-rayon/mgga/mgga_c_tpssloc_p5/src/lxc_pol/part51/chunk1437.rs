//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1437/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1437(t1992: f64, t33249: f64, t80650: f64, t114264: f64, t115596: f64, t115601: f64, t120594: f64, t1323: f64, t1375: f64, t16439: f64, t2015: f64, t22656: f64, t24095: f64, t26224: f64, t26370: f64, t26989: f64, t27009: f64, t27114: f64, t33266: f64, t33323: f64, t3887: f64, t568: f64, t6963: f64, t7750: f64, t7925: f64, t8627: f64, t91505: f64) -> f64 {
    let t122370 = t1992 * t80650 * t33249;
    let t122375 = 2.0_f64 * t22656 * t7925 + 2.0_f64 * t1375 * t3887 * t27114 * t2015 - 0.38381794893125283518e-1_f64 * t115596 + t114264 - 6.0_f64 * t91505 * t33323 - t120594 + t1323 * t33266 * t568 + 2.0_f64 * t27009 * t6963 - 6.0_f64 * t26224 * t26989 * t26370 + 0.41123351671205660912e-2_f64 * t115601 + 0.16449340668482264365e-1_f64 * t122370 + 2.0_f64 * t16439 * t8627 - t24095 * t7750;
    t122375
}
