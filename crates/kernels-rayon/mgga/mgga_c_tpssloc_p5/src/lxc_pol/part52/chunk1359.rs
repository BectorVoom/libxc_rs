//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1359/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1359(t12725: f64, t8323: f64, t55353: f64, t8319: f64, t16524: f64, t31280: f64, t23880: f64, t26550: f64, t33185: f64, t23877: f64, t7467: f64, t7769: f64, t83980: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t120753 = t12725 * t8323;
    let t120786 = 27.0_f64 * t55353 * t8319;
    let t120788 = 54.0_f64 * t16524 * t31280;
    let t120789 = t23880 * t26550;
    let t120792 = 54.0_f64 * t33185 * t31280;
    let t120793 = t23877 * t7467;
    let t120795 = t83980 * t7769;
    (t120753, t120786, t120788, t120789, t120792, t120793, t120795)
}
