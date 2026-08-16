//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1051/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1051(t28: f64, t6665: f64, t1081: f64, t1877: f64, t2522: f64, t30753: f64, t30757: f64, t30770: f64, t6670: f64, t6841: f64, t6848: f64, t8366: f64, t8370: f64) -> f64 {
    let t30974 = t28 * t6665;
    let t30982 = 3.0_f64 / 2.0_f64 * t2522 * t8366 * t6841 + t1877 * t30753 * t28 / 2.0_f64 - t1877 * t30757 * t6848 / 2.0_f64 + t1877 * t8366 * t1081 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t2522 * t8370 * t6841 - t1877 * t6670 * t30974 + t1877 * t30770 * t6848 - t1877 * t8370 * t1081 / 2.0_f64;
    t30982
}
