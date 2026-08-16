//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1057/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1057(t1863: f64, t26012: f64, t1410: f64, t2240: f64, t6505: f64, t7445: f64, t4017: f64, t71: f64, t12568: f64, t33: f64, t1409: f64, t22502: f64, t22505: f64, t22510: f64, t3961: f64, t3966: f64, t6500: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t26013 = t1863 * t26012;
    let t26016 = t2240 * t1410;
    let t26021 = t6505 * t7445;
    let t26024 = t71 * t4017;
    let t26025 = t1863 * t26024;
    let t26028 = t12568 * t33;
    let t26043 = -20.0_f64 / 9.0_f64 * t22502 * t1409 + 5.0_f64 / 18.0_f64 * t22505 * t3961 + 5.0_f64 / 6.0_f64 * t6500 * t3966 - t22510;
    (t26013, t26016, t26021, t26024, t26025, t26028, t26043)
}
