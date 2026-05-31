//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1419/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1419<F: Float>(t11282: F, t7785: F, t11315: F, t1161: F, t22531: F, t2829: F, t2869: F, t30571: F, t30682: F, t30686: F, t30689: F, t30692: F, t30697: F, t30703: F, t30710: F, t3739: F, t3747: F, t4512: F, t7637: F, t7643: F, t7800: F, t7806: F, t9587: F, t9594: F, t9657: F) -> (F, F) {
    let t30716 = t11282 * t7785;
    let t30719 = F::cast_from(512.0_f64) / F::cast_from(81.0_f64) * t30682 * t9657 + F::cast_from(5632.0_f64) / F::cast_from(2187.0_f64) * t9587 * t30686 + F::cast_from(704.0_f64) / F::cast_from(81.0_f64) * t3747 * t30689 + F::cast_from(1408.0_f64) / F::cast_from(243.0_f64) * t3739 * t30692 + F::cast_from(5632.0_f64) / F::cast_from(2187.0_f64) * t9594 * t30686 + F::cast_from(128.0_f64) / F::cast_from(3.0_f64) * t7806 * t30697 + F::cast_from(256.0_f64) / F::cast_from(81.0_f64) * t22531 * t30571 + F::cast_from(616.0_f64) / F::cast_from(9.0_f64) * t7637 * t30703 + F::cast_from(440.0_f64) / F::cast_from(9.0_f64) * t7800 * t1161 * t4512 * t2869 + F::cast_from(88.0_f64) / F::cast_from(9.0_f64) * t7643 * t30710 + F::cast_from(440.0_f64) / F::cast_from(9.0_f64) * t7800 * t11315 * t7785 - F::cast_from(88.0_f64) / F::cast_from(27.0_f64) * t2829 * t30716;
    (t30716, t30719)
}
