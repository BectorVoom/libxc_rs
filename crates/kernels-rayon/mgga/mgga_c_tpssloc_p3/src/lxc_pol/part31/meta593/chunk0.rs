//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1838/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1838(t1851: f64, t7240: f64, t1858: f64, t7222: f64, t26959: f64, t6495: f64, t26070: f64, t7032: f64, t26073: f64, t26076: f64, t23998: f64, t7435: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t91834 = 2.0_f64 * t1851 * t7240;
    let t91842 = 2.0_f64 * t7222 * t1858;
    let t91890 = 32.0_f64 / 9.0_f64 * t6495 * t26959;
    let t91894 = 32.0_f64 / 9.0_f64 * t26070 * t7032;
    let t91896 = 32.0_f64 / 9.0_f64 * t26073 * t7032;
    let t91898 = 32.0_f64 / 9.0_f64 * t26076 * t7032;
    let t91900 = 32.0_f64 / 9.0_f64 * t7435 * t23998;
    (t91834, t91842, t91890, t91894, t91896, t91898, t91900)
}
