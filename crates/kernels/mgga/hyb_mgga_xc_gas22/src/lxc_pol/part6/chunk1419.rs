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
    let t30719 = F::new(512.0) / F::new(81.0) * t30682 * t9657 + F::new(5632.0) / F::new(2187.0) * t9587 * t30686 + F::new(704.0) / F::new(81.0) * t3747 * t30689 + F::new(1408.0) / F::new(243.0) * t3739 * t30692 + F::new(5632.0) / F::new(2187.0) * t9594 * t30686 + F::new(128.0) / F::new(3.0) * t7806 * t30697 + F::new(256.0) / F::new(81.0) * t22531 * t30571 + F::new(616.0) / F::new(9.0) * t7637 * t30703 + F::new(440.0) / F::new(9.0) * t7800 * t1161 * t4512 * t2869 + F::new(88.0) / F::new(9.0) * t7643 * t30710 + F::new(440.0) / F::new(9.0) * t7800 * t11315 * t7785 - F::new(88.0) / F::new(27.0) * t2829 * t30716;
    (t30716, t30719)
}
