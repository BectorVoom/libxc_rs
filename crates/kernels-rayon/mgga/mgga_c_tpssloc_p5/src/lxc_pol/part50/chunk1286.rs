//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1286/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1286(t2022: f64, t7774: f64, t1851: f64, t8509: f64, t33196: f64, t576: f64, t55353: f64, t8319: f64, t16524: f64, t31280: f64, t23880: f64, t26550: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t120774 = t2022 * t7774;
    let t120780 = t1851 * t8509;
    let t120783 = t576 * t33196;
    let t120786 = 27.0_f64 * t55353 * t8319;
    let t120788 = 54.0_f64 * t16524 * t31280;
    let t120789 = t23880 * t26550;
    (t120774, t120780, t120783, t120786, t120788, t120789)
}
