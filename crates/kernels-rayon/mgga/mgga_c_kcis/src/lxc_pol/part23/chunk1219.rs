//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1219/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1219(t12345: f64, t4190: f64, t8207: f64, t28570: f64, t39301: f64, t17311: f64, t27509: f64, t12338: f64, t28580: f64, t17708: f64, t2253: f64, t4189: f64) -> (f64, f64, f64, f64, f64) {
    let t97852 = 6.0_f64 * t12345 * t8207 * t4190;
    let t97854 = 12.0_f64 * t39301 * t28570;
    let t97856 = 2.0_f64 * t17311 * t27509;
    let t97862 = 4.0_f64 * t12338 * t28580;
    let t97870 = 2.0_f64 * t4189 * t2253 * t17708;
    (t97852, t97854, t97856, t97862, t97870)
}
