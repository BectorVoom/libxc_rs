//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1359/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1359<F: Float>(t12725: F, t8323: F, t55353: F, t8319: F, t16524: F, t31280: F, t23880: F, t26550: F, t33185: F, t23877: F, t7467: F, t7769: F, t83980: F) -> (F, F, F, F, F, F, F) {
    let t120753 = t12725 * t8323;
    let t120786 = F::cast_from(27.0_f64) * t55353 * t8319;
    let t120788 = F::cast_from(54.0_f64) * t16524 * t31280;
    let t120789 = t23880 * t26550;
    let t120792 = F::cast_from(54.0_f64) * t33185 * t31280;
    let t120793 = t23877 * t7467;
    let t120795 = t83980 * t7769;
    (t120753, t120786, t120788, t120789, t120792, t120793, t120795)
}
