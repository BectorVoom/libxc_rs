//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1023/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1023(t20027: f64, t942: f64, t4458: f64, t4495: f64, t38571: f64, t85469: f64, t20224: f64, t11755: f64, t11756: f64, t11761: f64, t11762: f64, t11766: f64, t1587: f64, t16399: f64, t1787: f64, t20145: f64, t38570: f64, t46019: f64, t462: f64, t57980: f64, t73637: f64, t73653: f64, t73662: f64, t73675: f64, t73677: f64, t7750: f64, t8327: f64) -> (f64, f64, f64, f64) {
    let t86104 = t20027 * t942;
    let t86108 = t4458 * t4495;
    let t86121 = t38571 * t85469;
    let t86130 = t20224 * t942;
    let t86140 = -16.0_f64 / 3.0_f64 * t462 * t8327 * t86104 - 4.0_f64 * t462 * t1787 * t86108 + 16.0_f64 / 9.0_f64 * t57980 + 8.0_f64 * t462 * t1587 * t73653 * t942 - 36.0_f64 * t462 * t7750 * t16399 * t4495 - 80.0_f64 / 81.0_f64 * t462 * t38570 * t86121 + 112.0_f64 / 81.0_f64 * t46019 + 8.0_f64 / 3.0_f64 * t73637 - 8.0_f64 * t11761 * t11766 * t20145 - 8.0_f64 * t11761 * t11762 * t86130 + 8.0_f64 / 3.0_f64 * t11755 * t11756 * t86130 + 4.0_f64 / 9.0_f64 * t73662 + 40.0_f64 / 81.0_f64 * t73675 + 8.0_f64 / 3.0_f64 * t73677;
    (t86104, t86108, t86121, t86140)
}
