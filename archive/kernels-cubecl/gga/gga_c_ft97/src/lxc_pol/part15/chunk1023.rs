//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1023/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1023<F: Float>(t20027: F, t942: F, t4458: F, t4495: F, t38571: F, t85469: F, t20224: F, t11755: F, t11756: F, t11761: F, t11762: F, t11766: F, t1587: F, t16399: F, t1787: F, t20145: F, t38570: F, t46019: F, t462: F, t57980: F, t73637: F, t73653: F, t73662: F, t73675: F, t73677: F, t7750: F, t8327: F) -> (F, F, F, F) {
    let t86104 = t20027 * t942;
    let t86108 = t4458 * t4495;
    let t86121 = t38571 * t85469;
    let t86130 = t20224 * t942;
    let t86140 = -F::cast_from(16.0_f64) / F::cast_from(3.0_f64) * t462 * t8327 * t86104 - F::cast_from(4.0_f64) * t462 * t1787 * t86108 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t57980 + F::cast_from(8.0_f64) * t462 * t1587 * t73653 * t942 - F::cast_from(36.0_f64) * t462 * t7750 * t16399 * t4495 - F::cast_from(80.0_f64) / F::cast_from(81.0_f64) * t462 * t38570 * t86121 + F::cast_from(112.0_f64) / F::cast_from(81.0_f64) * t46019 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t73637 - F::cast_from(8.0_f64) * t11761 * t11766 * t20145 - F::cast_from(8.0_f64) * t11761 * t11762 * t86130 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t11755 * t11756 * t86130 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t73662 + F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t73675 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t73677;
    (t86104, t86108, t86121, t86140)
}
