//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 892/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk892(t1495: f64, t16721: f64, t4123: f64, t1464: f64, t3797: f64, t5632: f64, t1395: f64, t4153: f64, t4142: f64, t5776: f64, t11913: f64, t5650: f64) -> (f64, f64, f64, f64, f64) {
    let t16722 = t1495 * t16721;
    let t16723 = t4123 * t16722;
    let t16724 = t1464 * t16723;
    let t16726 = t5632 * t3797;
    let t16727 = t1395 * t16726;
    let t16728 = t4153 * t16727;
    let t16730 = t4142 * t5776;
    let t16731 = 0.22109259259259259258e-2_f64 * t16730;
    let t16732 = t11913 * t5650;
    (t16724, t16728, t16730, t16731, t16732)
}
