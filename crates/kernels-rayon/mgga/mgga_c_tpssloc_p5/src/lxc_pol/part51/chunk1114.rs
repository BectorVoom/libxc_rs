//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1114/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1114(t2031: f64, t26024: f64, t7032: f64, t7428: f64, t26012: f64, t7031: f64, t7445: f64, t1860: f64, t2032: f64, t22549: f64, t23963: f64, t23968: f64, t23970: f64, t23973: f64, t23978: f64, t23995: f64, t23999: f64, t26009: f64, t26016: f64, t26028: f64, t6486: f64, t7035: f64, t7782: f64) -> f64 {
    let t26945 = t2031 * t26024;
    let t26948 = t7428 * t7032;
    let t26954 = t2031 * t26012;
    let t26959 = t7031 * t7445;
    let t26960 = t1860 * t26959;
    let t26964 = t26028 * t2032 / 3.0_f64 + t7428 * t7035 / 3.0_f64 + t6486 * t7782 / 3.0_f64 + t1860 * t26945 / 3.0_f64 - 8.0_f64 / 9.0_f64 * t26948 - 8.0_f64 / 9.0_f64 * t23978 + t23995 - 8.0_f64 / 9.0_f64 * t23999 + 10.0_f64 * t23963 * t26009 + 10.0_f64 / 3.0_f64 * t22549 * t26954 + 10.0_f64 / 3.0_f64 * t26016 * t23970 - 8.0_f64 / 9.0_f64 * t26960 + 40.0_f64 / 9.0_f64 * t23968 + 16.0_f64 / 9.0_f64 * t23973;
    t26964
}
