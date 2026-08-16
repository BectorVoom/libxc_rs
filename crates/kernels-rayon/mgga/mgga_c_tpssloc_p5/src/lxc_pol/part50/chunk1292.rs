//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1292/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1292(t114449: f64, t114451: f64, t120774: f64, t120780: f64, t120783: f64, t120814: f64, t120852: f64, t1396: f64, t1398: f64, t1852: f64, t2023: f64, t26555: f64, t31288: f64, t33196: f64, t5364: f64, t7003: f64, t7020: f64, t7759: f64, t7774: f64, t8509: f64) -> f64 {
    let t120855 = 2.0_f64 * t7759 * t7020 + 2.0_f64 * t120774 + 2.0_f64 * t7003 * t7774 + 2.0_f64 * t2023 * t26555 + t114449 + t114451 + t120780 + t5364 * t8509 + t1852 * t31288 + t120783 + t1396 * t33196 + t1398 * (t120814 + t120852);
    t120855
}
