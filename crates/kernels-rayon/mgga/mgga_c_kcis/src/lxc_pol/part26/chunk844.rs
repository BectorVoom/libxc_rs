//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 844/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk844(t1444: f64, t16968: f64, t25: f64, t5733: f64, t493: f64, t11425: f64, t556: f64, t1404: f64, t4035: f64, t12048: f64, t5796: f64, t1401: f64, t5808: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16969 = t16968 * t1444;
    let t16979 = t25 * t5733;
    let t16981 = t493 * t16979 / 144.0_f64;
    let t17009 = t556 * t11425;
    let t17019 = t1404 * t4035;
    let t17024 = t12048 * t5796;
    let t17027 = 0.93706135855523581992e-2_f64 * t1401 * t5808;
    (t16969, t16981, t17009, t17019, t17024, t17027)
}
