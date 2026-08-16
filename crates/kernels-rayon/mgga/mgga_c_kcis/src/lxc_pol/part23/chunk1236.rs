//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1236/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1236(t1464: f64, t15910: f64, t3722: f64, t7923: f64, t28351: f64, t3715: f64, t51121: f64, t20905: f64, t27387: f64, t4136: f64, t16686: f64, t4153: f64) -> (f64, f64, f64, f64) {
    let t98171 = t1464 * t7923 * t15910 * t3722;
    let t98174 = t28351 * t51121 * t3715;
    let t98179 = t1464 * t27387 * t20905 * t4136;
    let t98188 = t4153 * t7923 * t16686;
    (t98171, t98174, t98179, t98188)
}
